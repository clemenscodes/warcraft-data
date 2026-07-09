//! Authoritative, hand-curated id tables that classify how particular abilities
//! behave on a unit's command card. These are database data — the only place
//! (besides the generated `db.rs`) allowed to mint ids from literals — so they
//! live here and are consumed by `warcraft-keybinds` as typed statics rather
//! than being fabricated from strings in the renderer/domain layer.

use crate::WarcraftObjectId;

/// Ability *codes* (the shared game-mechanic class, not the per-unit alias) that
/// only make sense on a building's rooted form and must be dropped from an
/// uprooted or otherwise non-rooted command card.
pub static ROOTED_ONLY_ABILITY_CODES: &[WarcraftObjectId] =
    &[WarcraftObjectId::new("Apit"), WarcraftObjectId::new("Aall")];

/// Specific ability ids that are rooted-form-only regardless of their code.
pub static ROOTED_ONLY_ABILITY_IDS: &[WarcraftObjectId] =
    &[WarcraftObjectId::new("Anei"), WarcraftObjectId::new("Aent")];

/// The Devour ability id. A building that can uproot hides Devour from its
/// rooted command card (it only applies to the mobile form).
pub const DEVOUR_ABILITY_ID: WarcraftObjectId = WarcraftObjectId::new("Aeat");

/// Root/uproot toggle ability ids that are structural fixtures of a building's
/// identity: the cascade treats their slot as pinned (always wins anchor
/// decisions, never a gap-pull candidate).
pub static PINNED_ROOT_ABILITY_IDS: &[WarcraftObjectId] =
    &[WarcraftObjectId::new("Aro1"), WarcraftObjectId::new("Aro2")];

/// A single (unit, ability) pair whose ability is deliberately hidden from that
/// unit's command card.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct HiddenUnitAbility {
    unit_id: WarcraftObjectId,
    ability_id: WarcraftObjectId,
}

impl HiddenUnitAbility {
    pub const fn new(unit_id: WarcraftObjectId, ability_id: WarcraftObjectId) -> Self {
        Self {
            unit_id,
            ability_id,
        }
    }

    pub fn unit_id(&self) -> WarcraftObjectId {
        self.unit_id
    }

    pub fn ability_id(&self) -> WarcraftObjectId {
        self.ability_id
    }
}

/// Abilities that must never be shown on the listed unit's command card.
pub static HIDDEN_UNIT_ABILITIES: &[HiddenUnitAbility] = &[
    HiddenUnitAbility::new(WarcraftObjectId::new("hphx"), WarcraftObjectId::new("Apxf")),
    HiddenUnitAbility::new(WarcraftObjectId::new("egol"), WarcraftObjectId::new("Aenc")),
];
