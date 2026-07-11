//! [`VariantGroup`]: the members of one logical unit that ships as several unit
//! ids, ordered weakest → strongest with the canonical form last. An internal
//! projection detail — consumers reach it only through the `UnitApi` variant
//! edges, never directly.

use crate::domain::identity::WarcraftObjectId;

/// A single logical unit that the game ships as several distinct unit ids —
/// leveled summon tiers (Carrion Beetle `ucs1`/`ucs2`/`ucs3`), upgrade-swaps
/// (Headhunter `ohun` upgraded into Berserker `otbk`), or a hero's duplicate
/// campaign/form ids (Alchemist `Nal2`/`Nal3`/`Nalm` behind `Nalc`). Members are
/// ordered weakest → strongest; the `canonical` — the form the editor shows and
/// edits fan out from — is last.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariantGroup {
    members: Vec<WarcraftObjectId>,
}

impl VariantGroup {
    /// Build a group from its already-ordered members (weakest → strongest).
    /// Only ever called with at least two members.
    pub(crate) fn new(members: Vec<WarcraftObjectId>) -> Self {
        Self { members }
    }

    /// Every member id, ordered weakest → strongest.
    pub(crate) fn members(&self) -> &[WarcraftObjectId] {
        &self.members
    }

    /// The canonical member — the strongest tier / upgraded unit / produced
    /// hero. Always the last member (groups always have at least two).
    pub(crate) fn canonical(&self) -> WarcraftObjectId {
        self.members.last().copied().unwrap_or_default()
    }
}
