//! [`SearchField`]: what a listing search query is matched against — the unit's
//! own name/id (default), or the abilities it carries.

/// The field a [`Search`](crate::application::unit::listing::query::Scope) scope
/// matches against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SearchField {
    /// Match the unit's own name and id (with a fuzzy fallback).
    #[default]
    UnitName,
    /// Match the names/ids of the abilities the unit carries — answers "which
    /// units have this ability?".
    Ability,
}
