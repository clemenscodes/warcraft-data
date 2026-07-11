//! The public read API over the bundled Warcraft III object database.

use crate::db::WARCRAFT_DATABASE;
use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::WarcraftObject;
use crate::infrastructure::database::WarcraftDatabase;

/// The single public entry point to the game data. Immutable and cheap to
/// construct (`WarcraftApi::default()`); every method reads through the
/// process-wide database, which is otherwise inaccessible.
///
/// ```
/// # use warcraft_api::WarcraftApi;
/// let api = WarcraftApi::default();
/// let _ = api.resolve("hpea").and_then(|id| api.object(id));
/// ```
#[derive(Clone, Copy, Debug)]
pub struct WarcraftApi {
    database: &'static WarcraftDatabase,
}

impl Default for WarcraftApi {
    fn default() -> Self {
        Self {
            database: &WARCRAFT_DATABASE,
        }
    }
}

impl WarcraftApi {
    /// Look up a known object by its already-typed id.
    pub fn object(&self, id: WarcraftObjectId) -> Option<&'static WarcraftObject> {
        self.database.object(id)
    }

    /// Resolve a genuinely-external raw id string (e.g. decoded from a URL) to
    /// its canonical [`WarcraftObjectId`]. This is the single sanctioned
    /// `&str`-in seam; every other lookup takes an already-typed id.
    pub fn resolve(&self, raw_id: &str) -> Option<WarcraftObjectId> {
        self.database
            .by_id_and_key(raw_id)
            .map(|(object_id, _object)| object_id)
    }

    /// Resolve a raw id string directly to its stored object.
    pub fn by_id(&self, raw_id: &str) -> Option<&'static WarcraftObject> {
        self.database.by_id(raw_id)
    }

    /// Iterate every known object with its id.
    pub fn iter(&self) -> impl Iterator<Item = (&'static WarcraftObjectId, &'static WarcraftObject)> {
        self.database.iter()
    }

    /// Number of objects in the database.
    pub fn len(&self) -> usize {
        self.database.iter().count()
    }

    /// Whether the database is empty (it never is in practice).
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // --- derived queries (formerly `ObjectLookup`) ---

    /// Whether the object has at least one displayable (non-blacklisted) icon.
    pub fn has_icon(&self, id: WarcraftObjectId) -> bool {
        self.object(id)
            .is_some_and(|object| object.has_displayable_icon())
    }

    /// Whether the object is a passive ability (passive icon convention).
    pub fn is_passive_ability(&self, id: WarcraftObjectId) -> bool {
        self.object(id)
            .is_some_and(|object| object.is_passive_ability())
    }

    /// The unit an ability morphs its caster into, if it is a one-way morph.
    pub fn morph_target_unit(&self, id: WarcraftObjectId) -> Option<WarcraftObjectId> {
        self.object(id)
            .and_then(|object| object.ability_morph_target_id())
    }

    /// The ability's game-mechanic code id, if the object is an ability.
    pub fn ability_code(&self, id: WarcraftObjectId) -> Option<WarcraftObjectId> {
        self.object(id).and_then(|object| object.ability_code())
    }

    /// The ability's off-state icon path, if any.
    pub fn off_icon(&self, id: WarcraftObjectId) -> Option<&'static str> {
        self.object(id).and_then(|object| object.ability_off_icon())
    }
}

// DDD role: the application service that reads the domain from infrastructure.
impl ddd::Layered for WarcraftApi {
    type Layer = ddd::ApplicationLayer;
}
impl ddd::ApplicationService for WarcraftApi {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_and_looks_up_a_known_unit() {
        let api = WarcraftApi::default();
        let id = api.resolve("hpea").expect("peasant exists");
        assert!(api.object(id).is_some());
    }

    #[test]
    fn resolve_is_case_insensitive() {
        let api = WarcraftApi::default();
        assert_eq!(api.resolve("HPEA"), api.resolve("hpea"));
    }

    #[test]
    fn unknown_id_resolves_to_none() {
        let api = WarcraftApi::default();
        assert!(api.resolve("this-is-not-a-real-id").is_none());
    }
}
