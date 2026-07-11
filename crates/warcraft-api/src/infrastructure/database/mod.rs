//! The backing object store. A case-insensitive, id-keyed catalog of every
//! known [`WarcraftObject`]. This is infrastructure — the application layer
//! (`WarcraftApi`) reads it; it is never exposed directly.
//!
//! The auto-generated [`generated`] submodule holds the process-wide static
//! data (the object catalog plus the gameplay-constant/system-keybind tables)
//! emitted by `warcraft-extractor`.

pub(crate) mod generated;

use std::collections::{BTreeMap, HashMap};

use warcraft_primitives::Identifier;

use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::{WarcraftObject, WarcraftObjectKind, WarcraftObjectMeta};

pub type ObjectMap = BTreeMap<WarcraftObjectId, WarcraftObject>;

#[derive(Default, Debug, Clone)]
pub struct WarcraftDatabase {
    db: ObjectMap,
    lowercase_index: HashMap<String, WarcraftObjectId>,
}

impl<'a> IntoIterator for &'a WarcraftDatabase {
    type Item = (&'a WarcraftObjectId, &'a WarcraftObject);
    type IntoIter = std::collections::btree_map::Iter<'a, WarcraftObjectId, WarcraftObject>;

    fn into_iter(self) -> Self::IntoIter {
        self.db.iter()
    }
}

impl WarcraftDatabase {
    pub fn new(db: ObjectMap) -> Self {
        let lowercase_index = db
            .keys()
            .map(|key| (key.value().to_ascii_lowercase(), *key))
            .collect();
        Self {
            db,
            lowercase_index,
        }
    }

    pub fn get(&self, id: Identifier) -> Option<&WarcraftObject> {
        let needle_id = id.get_id();
        self.by_id(needle_id.as_str())
    }

    pub fn db(&self) -> &ObjectMap {
        &self.db
    }

    /// Look up a known object by its already-typed id. Indexes the object map
    /// directly through the (case-insensitive) [`WarcraftObjectId`] key.
    pub fn object(&self, id: WarcraftObjectId) -> Option<&WarcraftObject> {
        self.db.get(&id)
    }

    /// Resolve a genuinely-external raw id string to its stored object. This and
    /// [`Self::by_id_and_key`] are the only `&str`-in seam: they fold ASCII case
    /// through `lowercase_index` because a runtime `&str` cannot be turned into a
    /// `WarcraftObjectId` (which only the database may mint).
    pub fn by_id(&self, needle_id: &str) -> Option<&WarcraftObject> {
        let lowercase = needle_id.to_ascii_lowercase();
        let canonical_key = self.lowercase_index.get(&lowercase)?;
        self.db.get(canonical_key)
    }

    pub fn by_id_and_key(&self, needle_id: &str) -> Option<(WarcraftObjectId, &WarcraftObject)> {
        let lowercase = needle_id.to_ascii_lowercase();
        let canonical_key = self.lowercase_index.get(&lowercase)?;
        let warcraft_object = self.db.get(canonical_key)?;
        Some((*canonical_key, warcraft_object))
    }

    pub fn get_icons(&self, id: Identifier) -> Option<&'static [&'static str]> {
        self.get(id).map(|object| object.icons())
    }

    pub fn get_names(&self, id: Identifier) -> Option<&'static [&'static str]> {
        self.get(id).map(|object| object.names())
    }

    pub fn get_ability_max_level(&self, id: Identifier) -> Option<usize> {
        match self.get(id)?.meta() {
            WarcraftObjectMeta::Ability(meta) => Some(meta.max_level()),
            _ => None,
        }
    }

    pub fn get_upgrade_max_level(&self, id: Identifier) -> Option<usize> {
        match self.get(id)?.meta() {
            WarcraftObjectMeta::Upgrade(meta) => Some(meta.max_level()),
            _ => None,
        }
    }

    pub fn get_max_level(&self, id: Identifier) -> Option<usize> {
        match self.get(id)?.meta() {
            WarcraftObjectMeta::Ability(meta) => Some(meta.max_level()),
            WarcraftObjectMeta::Upgrade(meta) => Some(meta.max_level()),
            _ => None,
        }
    }

    pub fn is_ultimate_ability(&self, id: Identifier) -> Option<bool> {
        match self.get(id)?.meta() {
            WarcraftObjectMeta::Ability(meta) => Some(meta.is_ultimate()),
            _ => None,
        }
    }

    pub fn get_ability_cooldown_for_level(&self, id: Identifier, level: usize) -> Option<u32> {
        match self.get(id)?.meta() {
            WarcraftObjectMeta::Ability(meta) => meta.cooldown_for_level(level),
            _ => None,
        }
    }

    pub fn get_ability_base_cooldown(&self, id: Identifier) -> Option<u32> {
        match self.get(id)?.meta() {
            WarcraftObjectMeta::Ability(meta) => Some(meta.base_cooldown()),
            _ => None,
        }
    }

    pub fn get_ability_cooldowns(&self, id: Identifier) -> Option<[u32; 4]> {
        match self.get(id)?.meta() {
            WarcraftObjectMeta::Ability(meta) => Some(meta.cooldowns()),
            _ => None,
        }
    }

    pub fn get_unit_build_time(&self, id: Identifier) -> Option<u32> {
        match self.get(id)?.meta() {
            WarcraftObjectMeta::Unit(meta) => Some(meta.build_time()),
            _ => None,
        }
    }

    pub fn ability_names(
        &self,
    ) -> impl Iterator<Item = (WarcraftObjectId, &'static [&'static str])> {
        self.db.iter().filter_map(|(id, object)| {
            if object.kind() == WarcraftObjectKind::Ability {
                Some((*id, object.names()))
            } else {
                None
            }
        })
    }

    pub fn all_ability_names(&'static self) -> impl Iterator<Item = &'static str> {
        self.db.values().filter_map(|object| {
            if object.kind() == WarcraftObjectKind::Ability {
                object.names().first().copied()
            } else {
                None
            }
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (&WarcraftObjectId, &WarcraftObject)> {
        self.db.iter()
    }
}
