//! Command-card command assembly: turns a unit's meta into the ordered list of
//! command-card buttons it shows in game. An `ApplicationService` — it reads the
//! object database through `WarcraftApi` and composes commands.

use std::sync::LazyLock;

use crate::{Race, UnitKind, UnitMeta, WarcraftObjectId, WarcraftObjectKind};

use crate::WarcraftApi;
use crate::domain::building::BuildingTraits;

const CONTEXT_COMMAND_IDS: &[WarcraftObjectId] = &[
    WarcraftObjectId::new("CmdCancel"),
    WarcraftObjectId::new("CmdCancelBuild"),
    WarcraftObjectId::new("CmdCancelRevive"),
    WarcraftObjectId::new("CmdCancelTrain"),
];

pub struct CommandCatalog;

impl CommandCatalog {
    fn build_command_for_race(race: Option<Race>) -> Option<WarcraftObjectId> {
        let race_value = race?;
        let preferred_command = match race_value {
            Race::Human => WarcraftObjectId::new("CmdBuildHuman"),
            Race::Orc => WarcraftObjectId::new("CmdBuildOrc"),
            Race::Nightelf => WarcraftObjectId::new("CmdBuildNightElf"),
            Race::Undead => WarcraftObjectId::new("CmdBuildUndead"),
            Race::Neutral => WarcraftObjectId::new("CmdBuild"),
        };
        let fallback_command = WarcraftObjectId::new("CmdBuild");
        Self::known_command(preferred_command).or_else(|| Self::known_command(fallback_command))
    }

    pub fn known_command(wanted_command: WarcraftObjectId) -> Option<WarcraftObjectId> {
        let warcraft_object = WarcraftApi::default().object(wanted_command)?;
        if warcraft_object.kind() == WarcraftObjectKind::Command {
            Some(warcraft_object.id())
        } else {
            None
        }
    }

    pub fn is_context_command_id(command_name: WarcraftObjectId) -> bool {
        CONTEXT_COMMAND_IDS.contains(&command_name)
    }

    pub fn submenu_back_command() -> Option<WarcraftObjectId> {
        let cancel_command = WarcraftObjectId::new("CmdCancel");
        Self::known_command(cancel_command)
    }

    pub fn select_skill_command() -> Option<WarcraftObjectId> {
        let select_skill_command = WarcraftObjectId::new("CmdSelectSkill");
        Self::known_command(select_skill_command)
    }

    pub fn mobile_command_ids() -> &'static [WarcraftObjectId] {
        MOBILE_COMMAND_IDS.as_slice()
    }

    pub fn primary_commands_for(
        unit_meta: &UnitMeta,
        race: Option<Race>,
        object_id: WarcraftObjectId,
    ) -> Vec<WarcraftObjectId> {
        let unit_kind = unit_meta.effective_kind();
        let has_builds = !unit_meta.builds().is_empty();
        let has_trains = !unit_meta.trains().is_empty();
        let has_production = has_builds || has_trains;
        let rally_command = WarcraftObjectId::new("CmdRally");
        let cancel_train_command = WarcraftObjectId::new("CmdCancelTrain");
        let mut commands: Vec<WarcraftObjectId> = Vec::new();
        match unit_kind {
            UnitKind::Building => {
                if BuildingTraits::can_attack(object_id) {
                    for command_name in TOWER_COMMAND_IDS.iter().copied() {
                        commands.push(command_name);
                    }
                }
                for command_name in BUILDING_COMMAND_IDS.iter().copied() {
                    if command_name == rally_command && !has_production {
                        continue;
                    }
                    if command_name == cancel_train_command && !has_production {
                        continue;
                    }
                    commands.push(command_name);
                }
            }
            UnitKind::Worker | UnitKind::Soldier | UnitKind::Hero => {
                for command_name in MOBILE_COMMAND_IDS.iter().copied() {
                    commands.push(command_name);
                }
                let attack_targets_ground = unit_meta
                    .combat()
                    .attack()
                    .is_some_and(|unit_attack| unit_attack.targets_ground());
                let attack_ground_command_id = WarcraftObjectId::new("CmdAttackGround");
                if attack_targets_ground
                    && let Some(attack_ground_command) =
                        Self::known_command(attack_ground_command_id)
                {
                    commands.push(attack_ground_command);
                }
                if has_builds
                    && unit_kind == UnitKind::Worker
                    && let Some(build_command) = Self::build_command_for_race(race)
                {
                    commands.insert(0, build_command);
                }
            }
        }
        commands.retain(|command_name| !Self::is_context_command_id(*command_name));
        commands
    }

    pub fn build_menu_commands_for(unit_meta: &UnitMeta) -> Vec<WarcraftObjectId> {
        if unit_meta.effective_kind() != UnitKind::Worker {
            return Vec::new();
        }
        if unit_meta.builds().is_empty() {
            return Vec::new();
        }
        Self::submenu_back_command().into_iter().collect()
    }
}

static MOBILE_COMMAND_IDS: LazyLock<Vec<WarcraftObjectId>> = LazyLock::new(|| {
    [
        WarcraftObjectId::new("CmdAttack"),
        WarcraftObjectId::new("CmdMove"),
        WarcraftObjectId::new("CmdStop"),
        WarcraftObjectId::new("CmdHoldPos"),
        WarcraftObjectId::new("CmdPatrol"),
    ]
    .into_iter()
    .filter_map(CommandCatalog::known_command)
    .collect()
});

static BUILDING_COMMAND_IDS: LazyLock<Vec<WarcraftObjectId>> = LazyLock::new(|| {
    [
        WarcraftObjectId::new("CmdCancelTrain"),
        WarcraftObjectId::new("CmdRally"),
    ]
    .into_iter()
    .filter_map(CommandCatalog::known_command)
    .collect()
});

static TOWER_COMMAND_IDS: LazyLock<Vec<WarcraftObjectId>> = LazyLock::new(|| {
    [
        WarcraftObjectId::new("CmdAttack"),
        WarcraftObjectId::new("CmdStop"),
    ]
    .into_iter()
    .filter_map(CommandCatalog::known_command)
    .collect()
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WarcraftObjectMeta;

    #[test]
    fn known_command_returns_some_for_cmd_attack() {
        let result = CommandCatalog::known_command(WarcraftObjectId::new("CmdAttack"));
        assert!(result.is_some());
    }

    #[test]
    fn known_command_returns_none_for_unknown() {
        let result = CommandCatalog::known_command(WarcraftObjectId::new("ZZZNotACommand"));
        assert!(result.is_none());
    }

    #[test]
    fn known_command_is_case_insensitive() {
        let lower = CommandCatalog::known_command(WarcraftObjectId::new("cmdattack"));
        let upper = CommandCatalog::known_command(WarcraftObjectId::new("CMDATTACK"));
        assert!(lower.is_some());
        assert!(upper.is_some());
    }

    #[test]
    fn is_context_command_id_true_for_cancel() {
        assert!(CommandCatalog::is_context_command_id(
            WarcraftObjectId::new("CmdCancel")
        ));
    }

    #[test]
    fn is_context_command_id_false_for_attack() {
        assert!(!CommandCatalog::is_context_command_id(
            WarcraftObjectId::new("CmdAttack")
        ));
    }

    #[test]
    fn mobile_command_ids_contains_attack_and_move() {
        let ids = CommandCatalog::mobile_command_ids();
        let attack_command = WarcraftObjectId::new("CmdAttack");
        let move_command = WarcraftObjectId::new("CmdMove");
        let has_attack = ids.contains(&attack_command);
        let has_move = ids.contains(&move_command);
        assert!(has_attack, "mobile commands must include CmdAttack");
        assert!(has_move, "mobile commands must include CmdMove");
    }

    fn primary_commands_for_unit(unit_id: &str) -> Vec<WarcraftObjectId> {
        let warcraft_object = WarcraftApi::default().by_id(unit_id).expect("unit exists");
        let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
            panic!("{unit_id} is not a unit");
        };
        let race = warcraft_object.race();
        let unit_object_id = warcraft_object.id();
        CommandCatalog::primary_commands_for(unit_meta, race, unit_object_id)
    }

    #[test]
    fn mortar_team_command_card_includes_attack_ground() {
        let commands = primary_commands_for_unit("hmtm");
        let attack_ground_command = WarcraftObjectId::new("CmdAttackGround");
        let has_attack_ground = commands.contains(&attack_ground_command);
        assert!(
            has_attack_ground,
            "the Mortar Team has an artillery weapon and must show Attack Ground, got {commands:?}"
        );
    }

    #[test]
    fn demolisher_command_card_includes_attack_ground() {
        let commands = primary_commands_for_unit("ocat");
        let attack_ground_command = WarcraftObjectId::new("CmdAttackGround");
        let has_attack_ground = commands.contains(&attack_ground_command);
        assert!(has_attack_ground, "the Demolisher must show Attack Ground");
    }

    #[test]
    fn footman_command_card_omits_attack_ground() {
        let commands = primary_commands_for_unit("hfoo");
        let attack_ground_command = WarcraftObjectId::new("CmdAttackGround");
        let has_attack_ground = commands.contains(&attack_ground_command);
        assert!(
            !has_attack_ground,
            "the Footman has a normal weapon and must not show Attack Ground, got {commands:?}"
        );
    }

    #[test]
    fn cannon_tower_command_card_omits_attack_ground() {
        let commands = primary_commands_for_unit("hctw");
        let attack_ground_command = WarcraftObjectId::new("CmdAttackGround");
        let has_attack_ground = commands.contains(&attack_ground_command);
        assert!(
            !has_attack_ground,
            "the Cannon Tower is a building and must not show Attack Ground, got {commands:?}"
        );
    }

    #[test]
    fn submenu_back_command_returns_cmd_cancel() {
        let result = CommandCatalog::submenu_back_command();
        assert!(result.is_some());
        let command_name = result.unwrap();
        let cancel_command = WarcraftObjectId::new("CmdCancel");
        assert_eq!(command_name, cancel_command);
    }
}

// DDD role: application service composing command-card commands.
impl ddd::Layered for CommandCatalog {
    type Layer = ddd::ApplicationLayer;
}
impl ddd::ApplicationService for CommandCatalog {}
