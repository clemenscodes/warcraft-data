//! The display essentials of a warcraft object — its name and its icon's
//! database path — resolved by id. Several list/detail views used to call
//! `WarcraftApi::default().by_id` and reach into `names()`/`icons()` themselves at render
//! time; that lookup is domain work (ARCHITECTURE R3), so it lives here. The
//! renderer keeps only presentation: turning the icon path into a URL and
//! choosing a fallback when the object has no name.

use warcraft_api::WarcraftObjectId;
use warcraft_api::WarcraftApi;

/// A warcraft object's name and icon database path, if it has them. Both are
/// `None` for an unknown id.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ObjectDisplay {
    name: Option<String>,
    icon_database_path: Option<String>,
}

impl ObjectDisplay {
    /// Resolve the object with the given id to its name and icon database path.
    pub fn resolve(object_id: WarcraftObjectId) -> Self {
        let object_option = WarcraftApi::default().object(object_id);
        let name = object_option
            .and_then(|object| object.names().first().copied())
            .map(|resolved_name| resolved_name.to_owned());
        let icon_database_path = object_option
            .and_then(|object| object.icons().first().copied())
            .map(|resolved_icon| resolved_icon.to_owned());
        Self {
            name,
            icon_database_path,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn icon_database_path(&self) -> Option<&str> {
        self.icon_database_path.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn footman_resolves_a_name_and_icon() {
        let display = ObjectDisplay::resolve(crate::test_support::object_id("hfoo"));
        assert!(display.name().is_some(), "the footman should have a name");
        assert!(
            display.icon_database_path().is_some(),
            "the footman should have an icon"
        );
    }

    #[test]
    fn unknown_string_is_rejected_by_the_database() {
        // Ids can only be obtained from the database, so an arbitrary string can
        // never become a `WarcraftObjectId` in the first place — the database
        // refuses to resolve it. This is the reachable guarantee that replaces
        // the old "resolve a fabricated unknown id" test.
        assert!(warcraft_api::WarcraftApi::default().resolve("this-is-not-a-real-object-id").is_none());
    }
}
