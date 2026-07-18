//! [`UnitQuery`] and [`Scope`]: the inputs to a unit listing. `Scope` makes the
//! browse-vs-search distinction an explicit, exhaustively-matched enum rather
//! than an inferred flag, so the sort order can never be tripped by accident.

use crate::application::unit::listing::mode_selection::UnitModeSelection;
use crate::application::unit::listing::race_selection::RaceSelection;
use crate::application::unit::listing::search_field::SearchField;
use crate::application::unit::listing::visibility::CatalogVisibility;
use crate::domain::unit::UnitKind;

/// A unit listing request. All fields optional via `Default`; construct with a
/// struct literal and `..Default::default()`.
#[derive(Debug, Clone, Default)]
pub struct UnitQuery<'a> {
    /// Which races this listing admits.
    pub races: RaceSelection,
    /// Restrict to one effective kind (all kinds when `None`).
    pub kind: Option<UnitKind>,
    /// The browsing simplifications to apply.
    pub visibility: CatalogVisibility,
    /// Whether this is a mode-selected browse or a cross-mode search.
    pub scope: Scope<'a>,
}

/// A listing is either a browse (of the selected game modes) or a search (across
/// every mode, matching a query against a field).
#[derive(Debug, Clone)]
pub enum Scope<'a> {
    /// Browse the units of the selected game modes.
    Browse { modes: UnitModeSelection },
    /// Search across modes for `query`, matched against `field`.
    Search { field: SearchField, query: &'a str },
}

impl Scope<'_> {
    /// The modes this scope lists. A search never narrows by mode: a hit may be
    /// a campaign unit even while the browse behind it shows melee, so it always
    /// spans both.
    pub fn modes(&self) -> UnitModeSelection {
        match self {
            Self::Browse { modes } => *modes,
            Self::Search { .. } => UnitModeSelection::both(),
        }
    }
}

impl Default for Scope<'_> {
    fn default() -> Self {
        let modes = UnitModeSelection::default();
        Self::Browse { modes }
    }
}
