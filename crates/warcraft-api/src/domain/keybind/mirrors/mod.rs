//! Hard-coded keybind mirror tables: a handful of build/morph commands whose
//! live-game keybind lives in a section other than the one the editor's grid
//! button edits. Normalization (in `warcraft-keybinds`) copies the edited
//! section onto its mirror so the game honors the position. These are
//! deliberately hand-listed database data — the authoritative place that mints
//! ids from literals — so they live here and are consumed as typed statics.

use crate::WarcraftObjectId;

/// Pairs a `CmdBuild*` command with the build ability whose keybind the live
/// game actually reads. The ability (e.g. `AHbu`) is what plays in game; the
/// `CmdBuild*` command only drives the in-game hotkey editor. Moving the build
/// command in the editor must write both, so the live game honors the position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct BuildCommandMirror {
    command_id: WarcraftObjectId,
    ability_id: WarcraftObjectId,
}

impl BuildCommandMirror {
    const fn new(command_id: WarcraftObjectId, ability_id: WarcraftObjectId) -> Self {
        Self {
            command_id,
            ability_id,
        }
    }

    pub fn command_id(&self) -> WarcraftObjectId {
        self.command_id
    }

    pub fn ability_id(&self) -> WarcraftObjectId {
        self.ability_id
    }
}

pub static BUILD_COMMAND_MIRRORS: &[BuildCommandMirror] = &[
    BuildCommandMirror::new(
        WarcraftObjectId::new("CmdBuildHuman"),
        WarcraftObjectId::new("AHbu"),
    ),
    BuildCommandMirror::new(
        WarcraftObjectId::new("CmdBuildOrc"),
        WarcraftObjectId::new("AObu"),
    ),
    BuildCommandMirror::new(
        WarcraftObjectId::new("CmdBuildUndead"),
        WarcraftObjectId::new("AUbu"),
    ),
    BuildCommandMirror::new(
        WarcraftObjectId::new("CmdBuildNightElf"),
        WarcraftObjectId::new("AEbu"),
    ),
];

/// Pairs a permanent one-way morph ability with the produced-unit section the
/// live game reads its keybind from. The Obsidian Statue's Transform (`Aave`)
/// is irreversible — a Destroyer (`ubsp`) can never become a Statue again — so
/// the morph is a one-time command whose keybind lives in a section keyed by
/// the produced unit id, separate from the `Aave` ability the editor's grid
/// button edits. Editing the button only touches `Aave`, so without this mirror
/// the produced-unit section keeps its stale default hotkey and the morph binds
/// the wrong key in game.
///
/// This is why the list is a single entry and is not derived from the database:
/// every *other* morph is a reversible toggle whose second state is the base
/// unit's off-state (`Unhotkey`/`Unbuttonpos`), so it has no orphaned
/// produced-unit command section to sync. The `morph_target_unit` database
/// field cannot distinguish these — it is also set for reversible toggles,
/// summon spells, and mount actions, several of whose targets are ordinary
/// train/sell units that this mirror would clobber.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MorphAbilityMirror {
    ability_id: WarcraftObjectId,
    produced_unit_id: WarcraftObjectId,
}

impl MorphAbilityMirror {
    const fn new(ability_id: WarcraftObjectId, produced_unit_id: WarcraftObjectId) -> Self {
        Self {
            ability_id,
            produced_unit_id,
        }
    }

    pub fn ability_id(&self) -> WarcraftObjectId {
        self.ability_id
    }

    pub fn produced_unit_id(&self) -> WarcraftObjectId {
        self.produced_unit_id
    }
}

pub static MORPH_ABILITY_MIRRORS: &[MorphAbilityMirror] = &[MorphAbilityMirror::new(
    WarcraftObjectId::new("Aave"),
    WarcraftObjectId::new("ubsp"),
)];

impl ddd::Layered for BuildCommandMirror {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for BuildCommandMirror {}
impl ddd::Layered for MorphAbilityMirror {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for MorphAbilityMirror {}
