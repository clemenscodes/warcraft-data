use std::collections::HashMap;

use warcraft_api::WarcraftObjectId;

/// Union–find over ability slot ids, used to split the abilities sharing
/// one grid cell into independent collision islands. Two abilities are merged
/// when some unit carries both of them at that cell — the same edge rule the
/// cascade's conflict graph uses. Components that never merge share no carrier
/// unit and therefore never interact.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct SlotIslandPartition {
    parent: HashMap<WarcraftObjectId, WarcraftObjectId>,
}

impl SlotIslandPartition {
    pub(crate) fn new() -> Self {
        let parent = HashMap::new();
        Self { parent }
    }

    pub(crate) fn register(&mut self, slot_id: WarcraftObjectId) {
        let already_present = self.parent.contains_key(&slot_id);
        if already_present {
            return;
        }
        self.parent.insert(slot_id, slot_id);
    }

    pub(crate) fn root(&mut self, slot_id: WarcraftObjectId) -> WarcraftObjectId {
        self.register(slot_id);
        let mut current = slot_id;
        loop {
            let parent = self
                .parent
                .get(&current)
                .copied()
                .expect("a registered id always has a parent entry");
            if parent == current {
                return current;
            }
            current = parent;
        }
    }

    pub(crate) fn union(&mut self, left_id: WarcraftObjectId, right_id: WarcraftObjectId) {
        let left_root = self.root(left_id);
        let right_root = self.root(right_id);
        if left_root != right_root {
            self.parent.insert(left_root, right_root);
        }
    }
}
