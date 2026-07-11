//! [`UnitQuery`] and [`Scope`]: the inputs to a unit listing. `Scope` makes the
//! browse-vs-search distinction an explicit, exhaustively-matched enum rather
//! than an inferred flag, so the sort order can never be tripped by accident.

use crate::application::unit::listing::search_field::SearchField;
use crate::application::unit::listing::visibility::CatalogVisibility;
use crate::domain::race::Race;
use crate::domain::unit::{UnitKind, UnitMode};

/// A unit listing request. All fields optional via `Default`; construct with a
/// struct literal and `..Default::default()`.
#[derive(Debug, Clone, Default)]
pub struct UnitQuery<'a> {
    /// Restrict to one race (all races when `None`).
    pub race: Option<Race>,
    /// Restrict to one effective kind (all kinds when `None`).
    pub kind: Option<UnitKind>,
    /// The browsing simplifications to apply.
    pub visibility: CatalogVisibility,
    /// Whether this is a mode-filtered browse or a cross-mode search.
    pub scope: Scope<'a>,
}

/// A listing is either a browse (filtered to one game mode) or a search (across
/// modes, matching a query against a field).
#[derive(Debug, Clone)]
pub enum Scope<'a> {
    /// Browse the units of one game mode.
    Browse { mode: UnitMode },
    /// Search across modes for `query`, matched against `field`.
    Search { field: SearchField, query: &'a str },
}

impl Default for Scope<'_> {
    fn default() -> Self {
        Self::Browse {
            mode: UnitMode::Melee,
        }
    }
}
