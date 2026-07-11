//! [`VariantGroup`]: the members of one logical unit that ships as several unit
//! ids. An internal projection detail — consumers see it only indirectly, via
//! the `UnitApi` variant edges.

use crate::domain::identity::WarcraftObjectId;

/// A single logical unit that the game ships as several distinct unit ids —
/// leveled summon tiers (Carrion Beetle `ucs1`/`ucs2`/`ucs3`), upgrade-swaps
/// (Headhunter `ohun` upgraded into Berserker `otbk`), or a hero's duplicate
/// campaign/form ids (Alchemist `Nal2`/`Nal3`/`Nalm` behind `Nalc`). Members
/// are ordered so the `canonical` unit — the one the editor displays and that
/// edits fan out from — is last: the strongest tier, the upgraded unit, or the
/// produced (trained/sold) hero.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariantGroup {
    members: Vec<WarcraftObjectId>,
}

impl VariantGroup {
    /// Build a group from its members, ordered weakest → strongest (canonical
    /// last). Groups are always built with at least two members.
    pub(crate) fn new(members: Vec<WarcraftObjectId>) -> Self {
        Self { members }
    }

    /// Every member id, ordered weakest → strongest.
    pub(crate) fn members(&self) -> &[WarcraftObjectId] {
        &self.members
    }

    /// The canonical member — the one the editor shows and the fan-out target
    /// (strongest tier / upgraded unit / produced hero). It is always last.
    /// Groups are always built with at least two members, so a member always
    /// exists.
    pub(crate) fn canonical(&self) -> WarcraftObjectId {
        let strongest = self.members.last();
        strongest.copied().unwrap_or_default()
    }

    pub(crate) fn weaker_members(&self) -> &[WarcraftObjectId] {
        let member_count = self.members.len();
        let split_index = member_count.saturating_sub(1);
        &self.members[..split_index]
    }
}
