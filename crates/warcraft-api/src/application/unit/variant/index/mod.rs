//! [`VariantIndex`]: the built, queryable variant projection. A plain data
//! structure — the ordered groups plus the O(1) lookups derived from them. It
//! computes nothing beyond answering membership/canonical/hidden questions.

use std::collections::{HashMap, HashSet};

use crate::application::unit::variant::group::VariantGroup;
use crate::domain::identity::WarcraftObjectId;

pub(crate) struct VariantIndex {
    groups: Vec<VariantGroup>,
    group_of: HashMap<WarcraftObjectId, usize>,
    canonical_of: HashMap<WarcraftObjectId, WarcraftObjectId>,
    hidden: HashSet<WarcraftObjectId>,
}

impl VariantIndex {
    /// The group `id` belongs to, or `None` when it stands alone.
    pub(crate) fn group(&self, id: WarcraftObjectId) -> Option<&VariantGroup> {
        let group_index = self.group_of.get(&id).copied()?;
        self.groups.get(group_index)
    }

    /// The canonical (strongest/representative) sibling of `id`; the canonical
    /// maps to itself. `None` when `id` is in no group.
    pub(crate) fn canonical(&self, id: WarcraftObjectId) -> Option<WarcraftObjectId> {
        self.canonical_of.get(&id).copied()
    }

    /// True iff `id` is a member of some group and is not its canonical — a
    /// weaker form the editor hides.
    pub(crate) fn is_hidden(&self, id: WarcraftObjectId) -> bool {
        self.hidden.contains(&id)
    }

    /// Every variant group, ordered by canonical id. Used by the ability fanout
    /// projection.
    pub(crate) fn groups(&self) -> &[VariantGroup] {
        &self.groups
    }
}

/// Index a set of ordered groups: record, per member, its group, its canonical,
/// and whether it is a hidden (non-canonical) form.
impl From<Vec<VariantGroup>> for VariantIndex {
    fn from(groups: Vec<VariantGroup>) -> Self {
        let mut group_of: HashMap<WarcraftObjectId, usize> = HashMap::new();
        let mut canonical_of: HashMap<WarcraftObjectId, WarcraftObjectId> = HashMap::new();
        let mut hidden: HashSet<WarcraftObjectId> = HashSet::new();
        for (group_index, group) in groups.iter().enumerate() {
            let canonical = group.canonical();
            for member in group.members().iter().copied() {
                group_of.insert(member, group_index);
                canonical_of.insert(member, canonical);
                if member != canonical {
                    hidden.insert(member);
                }
            }
        }
        Self {
            groups,
            group_of,
            canonical_of,
            hidden,
        }
    }
}
