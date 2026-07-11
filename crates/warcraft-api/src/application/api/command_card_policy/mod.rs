//! Command-card policy: the derived rules a consumer needs to lay out a unit's
//! command card — which abilities are hidden, rooted-only, or morph back into
//! the host; which units start in a toggled alternate (burrowed / rooted /
//! militia) state; and the fixed menu-command buttons a build/research/uprooted
//! menu shows. These read the game catalog, so they live on the application
//! service beside the other derived object queries; the keybind editor consumes
//! them to place slots.

use super::WarcraftApi;
use crate::UNIT_UPGRADE_SWAPS;
use crate::domain::identity::WarcraftObjectId;
use crate::domain::keybind::ability_tables::{
    HIDDEN_UNIT_ABILITIES, ROOTED_ONLY_ABILITY_CODES, ROOTED_ONLY_ABILITY_IDS,
};
use crate::domain::object::{WarcraftObjectKind, WarcraftObjectMeta};
use crate::domain::unit::{UnitKind, UnitMeta};

/// The mobile-unit base command set, in card order.
const MOBILE_COMMANDS: &[&str] = &["CmdAttack", "CmdMove", "CmdStop", "CmdHoldPos", "CmdPatrol"];

impl WarcraftApi {
    /// Whether the ability is deliberately hidden from this unit's command card
    /// (an entry in [`HIDDEN_UNIT_ABILITIES`]).
    pub fn ability_is_hidden_on_unit(
        &self,
        unit_id: WarcraftObjectId,
        ability_id: WarcraftObjectId,
    ) -> bool {
        HIDDEN_UNIT_ABILITIES
            .iter()
            .any(|hidden| hidden.unit_id() == unit_id && hidden.ability_id() == ability_id)
    }

    /// Whether the two units are the same trainable button at different tech
    /// tiers — a genuine upgrade swap (e.g. Headhunter → Berserker). Two units
    /// that merely share a default button cell are not a swap.
    pub fn units_are_upgrade_swap(
        &self,
        first_unit_id: WarcraftObjectId,
        second_unit_id: WarcraftObjectId,
    ) -> bool {
        UNIT_UPGRADE_SWAPS.iter().any(|swap| {
            let from_id = swap.from_unit_id();
            let to_id = swap.to_unit_id();
            (from_id == first_unit_id && to_id == second_unit_id)
                || (from_id == second_unit_id && to_id == first_unit_id)
        })
    }

    /// Whether the morph ability collapses back into the given host unit, so it
    /// must not claim its own separate command slot. A morph that also carries a
    /// distinct off-state does keep its slot, so it is excluded.
    pub fn morph_reverts_to_host(
        &self,
        ability_id: WarcraftObjectId,
        host_unit_id: WarcraftObjectId,
    ) -> bool {
        let Some(target_id) = self
            .object(ability_id)
            .and_then(|object| object.ability_morph_target_id())
        else {
            return false;
        };
        if target_id != host_unit_id {
            return false;
        }
        !self.ability_has_alt_state(ability_id)
    }

    /// Whether the ability only exists on the unit's rooted form and must be
    /// dropped from an uprooted or otherwise non-rooted command card.
    pub fn ability_is_rooted_only(&self, ability_id: WarcraftObjectId) -> bool {
        if ROOTED_ONLY_ABILITY_IDS.contains(&ability_id) {
            return true;
        }
        let Some(ability_code) = self
            .object(ability_id)
            .and_then(|object| object.ability_code())
        else {
            return false;
        };
        ROOTED_ONLY_ABILITY_CODES.contains(&ability_code)
    }

    /// Whether the unit's first display name marks it as a burrowed form.
    pub fn is_burrowed_form(&self, unit_id: WarcraftObjectId) -> bool {
        let Some(object) = self.object(unit_id) else {
            return false;
        };
        let Some(first_name) = object.names().first().copied() else {
            return false;
        };
        first_name.to_ascii_lowercase().starts_with("burrowed ")
    }

    /// Whether the unit's default in-game state is the toggled alternate: an
    /// uprootable building, a burrowed form, or the Militia (`hmil`).
    pub fn unit_starts_in_toggle_alt_state(&self, unit_id: WarcraftObjectId) -> bool {
        self.unit().can_uproot(unit_id)
            || self.is_burrowed_form(unit_id)
            || self.resolve("hmil") == Some(unit_id)
    }

    /// Whether the ability appears on any unit that starts in a toggled
    /// alternate state.
    pub fn ability_is_on_alt_state_unit(&self, ability_id: WarcraftObjectId) -> bool {
        self.iter().any(|(unit_id, object)| {
            if !self.unit_starts_in_toggle_alt_state(*unit_id) {
                return false;
            }
            matches!(
                object.meta(),
                WarcraftObjectMeta::Unit(unit_meta) if unit_meta.abilities().contains(&ability_id)
            )
        })
    }

    /// Whether the ability carries alternate-state tooltip text (its `un_tip` /
    /// `un_ubertip`), the signal that it renders a distinct off-state.
    pub fn ability_has_alt_state(&self, ability_id: WarcraftObjectId) -> bool {
        let Some(object) = self.object(ability_id) else {
            return false;
        };
        object.un_tip().is_some() || object.un_ubertip().is_some()
    }

    /// The mobile-unit base command ids (attack / move / stop / hold / patrol),
    /// in card order.
    pub fn mobile_command_ids(&self) -> Vec<WarcraftObjectId> {
        self.known_commands(MOBILE_COMMANDS)
    }

    /// The "back" command shown in a submenu (`CmdCancel`).
    pub fn submenu_back_command(&self) -> Option<WarcraftObjectId> {
        self.known_command("CmdCancel")
    }

    /// The hero "select skill" command (`CmdSelectSkill`).
    pub fn select_skill_command(&self) -> Option<WarcraftObjectId> {
        self.known_command("CmdSelectSkill")
    }

    /// The build-menu commands for a building worker (just the back command).
    pub fn build_menu_commands(&self, unit_meta: &UnitMeta) -> Vec<WarcraftObjectId> {
        if unit_meta.effective_kind() != UnitKind::Worker {
            return Vec::new();
        }
        if unit_meta.builds().is_empty() {
            return Vec::new();
        }
        self.submenu_back_command().into_iter().collect()
    }

    /// Resolve a command id string to its canonical id, but only when it exists
    /// as a command object in the database.
    fn known_command(&self, command: &str) -> Option<WarcraftObjectId> {
        let object = self.by_id(command)?;
        (object.kind() == WarcraftObjectKind::Command).then(|| object.id())
    }

    /// The known command ids of a string table, preserving order.
    fn known_commands(&self, table: &[&str]) -> Vec<WarcraftObjectId> {
        table
            .iter()
            .filter_map(|command| self.known_command(command))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::WarcraftApi;
    use crate::domain::identity::WarcraftObjectId;

    fn id(value: &'static str) -> WarcraftObjectId {
        WarcraftObjectId::new(value)
    }

    #[test]
    fn hidden_ability_matches_only_the_listed_pairs() {
        let api = WarcraftApi::default();
        assert!(api.ability_is_hidden_on_unit(id("hphx"), id("Apxf")));
        assert!(!api.ability_is_hidden_on_unit(id("hpea"), id("Apxf")));
    }

    #[test]
    fn rooted_only_ability_flags_a_known_rooted_ability() {
        let api = WarcraftApi::default();
        assert!(api.ability_is_rooted_only(id("Anei")));
        assert!(!api.ability_is_rooted_only(id("AHbz")));
    }
}
