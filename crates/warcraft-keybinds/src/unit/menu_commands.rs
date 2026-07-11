//! Keybind-local menu/submenu command policy: the fixed command-id buttons a
//! unit's build menu, research submenu, uprooted menu, and hero card show. This
//! is keybind-specific command-grid policy, so it lives here rather than in the
//! general `warcraft-api`. Command ids are resolved through a [`WarcraftApi`]
//! handle (keybinds cannot mint object ids from strings).

use warcraft_api::{UnitKind, UnitMeta, WarcraftApi, WarcraftObjectId, WarcraftObjectKind};

/// The mobile-unit base command set, in card order.
const MOBILE_COMMANDS: &[&str] = &["CmdAttack", "CmdMove", "CmdStop", "CmdHoldPos", "CmdPatrol"];

/// Resolve a command id string to its canonical id, but only when it exists as a
/// command object in the database.
fn known_command(api: WarcraftApi, command: &str) -> Option<WarcraftObjectId> {
    let object = api.by_id(command)?;
    if object.kind() == WarcraftObjectKind::Command {
        Some(object.id())
    } else {
        None
    }
}

/// The known command ids of a string table, preserving order.
fn known_commands(api: WarcraftApi, table: &[&str]) -> Vec<WarcraftObjectId> {
    table
        .iter()
        .filter_map(|command| known_command(api, command))
        .collect()
}

/// Fixed command-id buttons shown in a unit's menus and submenus. Implemented
/// for [`WarcraftApi`] so the command strings resolve against the database.
pub trait MenuCommands {
    /// The mobile-unit base command ids (attack / move / stop / hold / patrol).
    fn mobile_command_ids(&self) -> Vec<WarcraftObjectId>;

    /// The "back" command shown in a submenu (`CmdCancel`).
    fn submenu_back_command(&self) -> Option<WarcraftObjectId>;

    /// The hero "select skill" command (`CmdSelectSkill`).
    fn select_skill_command(&self) -> Option<WarcraftObjectId>;

    /// The build-menu commands for a building worker (just the back command).
    fn build_menu_commands(&self, unit_meta: &UnitMeta) -> Vec<WarcraftObjectId>;
}

impl MenuCommands for WarcraftApi {
    fn mobile_command_ids(&self) -> Vec<WarcraftObjectId> {
        known_commands(*self, MOBILE_COMMANDS)
    }

    fn submenu_back_command(&self) -> Option<WarcraftObjectId> {
        known_command(*self, "CmdCancel")
    }

    fn select_skill_command(&self) -> Option<WarcraftObjectId> {
        known_command(*self, "CmdSelectSkill")
    }

    fn build_menu_commands(&self, unit_meta: &UnitMeta) -> Vec<WarcraftObjectId> {
        if unit_meta.effective_kind() != UnitKind::Worker {
            return Vec::new();
        }
        if unit_meta.builds().is_empty() {
            return Vec::new();
        }
        self.submenu_back_command().into_iter().collect()
    }
}
