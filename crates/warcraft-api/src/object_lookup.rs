use crate::{WarcraftObject, WarcraftObjectId};

use crate::WARCRAFT_DATABASE;

pub struct ObjectLookup;

impl ObjectLookup {
    /// Look up a known object by its already-typed id. Delegates to the
    /// database's typed, case-insensitive [`WarcraftObjectId`]-keyed index.
    /// Callers pass a `WarcraftObjectId`, never a string.
    pub fn object(id: WarcraftObjectId) -> Option<&'static WarcraftObject> {
        WARCRAFT_DATABASE.object(id)
    }

    /// Resolve a genuinely-external raw id string (e.g. one decoded from a URL)
    /// to its canonical [`WarcraftObjectId`]. This is the single sanctioned
    /// `&str`-in entry point; every other lookup takes an already-known id.
    pub fn resolve_raw(needle_id: &str) -> Option<WarcraftObjectId> {
        WARCRAFT_DATABASE
            .by_id_and_key(needle_id)
            .map(|(object_id, _object)| object_id)
    }

    pub fn has_icon(object_id: WarcraftObjectId) -> bool {
        let database_object = Self::object(object_id);
        database_object.is_some_and(|object| object.has_displayable_icon())
    }

    pub fn is_passive_ability(object_id: WarcraftObjectId) -> bool {
        let database_object = Self::object(object_id);
        database_object.is_some_and(|object| object.is_passive_ability())
    }

    pub fn morph_target_unit(object_id: WarcraftObjectId) -> Option<WarcraftObjectId> {
        let database_object = Self::object(object_id);
        database_object.and_then(|object| object.ability_morph_target_id())
    }

    pub fn ability_code(object_id: WarcraftObjectId) -> Option<WarcraftObjectId> {
        let database_object = Self::object(object_id);
        database_object.and_then(|object| object.ability_code())
    }

    pub fn off_icon(object_id: WarcraftObjectId) -> Option<&'static str> {
        let database_object = Self::object(object_id);
        database_object.and_then(|object| object.ability_off_icon())
    }
}
