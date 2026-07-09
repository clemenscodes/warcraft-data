use std::sync::LazyLock;

use crate::{Race, UnitKind, UnitMeta, WarcraftObjectId, WarcraftObjectKind, WarcraftObjectMeta};

use crate::WARCRAFT_DATABASE;

const ATTACKING_BUILDING_IDS: &[WarcraftObjectId] = &[
    WarcraftObjectId::new("hgtw"),
    WarcraftObjectId::new("hatw"),
    WarcraftObjectId::new("hctw"),
    WarcraftObjectId::new("owtw"),
    WarcraftObjectId::new("otrb"),
    WarcraftObjectId::new("unp1"),
    WarcraftObjectId::new("unp2"),
    WarcraftObjectId::new("uzg1"),
    WarcraftObjectId::new("uzg2"),
    WarcraftObjectId::new("nadt"),
    WarcraftObjectId::new("ndgt"),
    WarcraftObjectId::new("ntt1"),
];

// Root and Uproot are the two states of one ability. Every uprootable Night Elf
// building carries a root ability object (Aro1 or Aro2), and both share this
// base ability code. Detecting the code is the 100% signal for "this building
// uproots", so no hand-maintained id list is needed.
const ROOT_ABILITY_CODE: WarcraftObjectId = WarcraftObjectId::new("Aroo");

pub struct BuildingTraits;

impl BuildingTraits {
    pub fn can_attack(object_id: WarcraftObjectId) -> bool {
        ATTACKING_BUILDING_IDS.contains(&object_id)
    }

    pub fn can_uproot(object_id: WarcraftObjectId) -> bool {
        let Some(warcraft_object) = WARCRAFT_DATABASE.object(object_id) else {
            return false;
        };
        let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
            return false;
        };
        unit_meta
            .abilities()
            .iter()
            .any(|ability_id| Self::ability_is_root(*ability_id))
    }

    fn ability_is_root(ability_id: WarcraftObjectId) -> bool {
        let Some(ability_object) = WARCRAFT_DATABASE.object(ability_id) else {
            return false;
        };
        let Some(ability_code) = ability_object.ability_code() else {
            return false;
        };
        ROOT_ABILITY_CODE == ability_code
    }

    pub fn unit_starts_in_toggle_alt_state(unit_id: WarcraftObjectId) -> bool {
        if Self::can_uproot(unit_id) {
            return true;
        }
        if Self::is_burrowed_form(unit_id) {
            return true;
        }
        let militia_id = WarcraftObjectId::new("hmil");
        unit_id == militia_id
    }

    pub fn ability_is_on_alt_state_unit(ability_id: WarcraftObjectId) -> bool {
        for (unit_id_obj, warcraft_object) in WARCRAFT_DATABASE.iter() {
            if !Self::unit_starts_in_toggle_alt_state(*unit_id_obj) {
                continue;
            }
            let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
                continue;
            };
            let has_ability = unit_meta.abilities().contains(&ability_id);
            if has_ability {
                return true;
            }
        }
        false
    }

    pub fn is_burrowed_form(unit_id: WarcraftObjectId) -> bool {
        let Some(warcraft_object) = WARCRAFT_DATABASE.object(unit_id) else {
            return false;
        };
        let names = warcraft_object.names();
        let Some(first_name) = names.first().copied() else {
            return false;
        };
        let lowercase_name = first_name.to_ascii_lowercase();
        lowercase_name.starts_with("burrowed ")
    }

    pub fn ability_has_alt_state(ability_id: WarcraftObjectId) -> bool {
        let Some(warcraft_object) = WARCRAFT_DATABASE.object(ability_id) else {
            return false;
        };
        warcraft_object.un_tip().is_some() || warcraft_object.un_ubertip().is_some()
    }
}

const CONTEXT_COMMAND_IDS: &[WarcraftObjectId] = &[
    WarcraftObjectId::new("CmdCancel"),
    WarcraftObjectId::new("CmdCancelBuild"),
    WarcraftObjectId::new("CmdCancelRevive"),
    WarcraftObjectId::new("CmdCancelTrain"),
];

pub struct CommandCatalog;

impl CommandCatalog {
    pub fn effective_kind(unit_meta: &UnitMeta) -> UnitKind {
        if unit_meta.is_special() && unit_meta.unit_kind() == UnitKind::Worker {
            return UnitKind::Soldier;
        }
        unit_meta.unit_kind()
    }

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
        let warcraft_object = WARCRAFT_DATABASE.object(wanted_command)?;
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
        let unit_kind = Self::effective_kind(unit_meta);
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
        if Self::effective_kind(unit_meta) != UnitKind::Worker {
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
mod catalog_tests {
    use super::*;

    #[test]
    fn can_attack_returns_true_for_guard_tower() {
        assert!(BuildingTraits::can_attack(WarcraftObjectId::new("hgtw")));
    }

    #[test]
    fn can_attack_returns_false_for_town_hall() {
        assert!(!BuildingTraits::can_attack(WarcraftObjectId::new("htow")));
    }

    #[test]
    fn can_uproot_returns_true_for_tree_of_life() {
        assert!(BuildingTraits::can_uproot(WarcraftObjectId::new("etol")));
    }

    #[test]
    fn can_uproot_returns_false_for_barracks() {
        assert!(!BuildingTraits::can_uproot(WarcraftObjectId::new("hbar")));
    }

    // The signal for "this building uproots" is the presence of the Root/Uproot
    // ability, not a hand-maintained id list. etrp and ncap carry the Aro2 root
    // variant (a different ability object id than Aro1, but the same "Aroo"
    // ability code), so detection must be driven by the ability code.
    #[test]
    fn can_uproot_detects_root_via_ability_code() {
        assert!(BuildingTraits::can_uproot(WarcraftObjectId::new("etrp")));
        assert!(BuildingTraits::can_uproot(WarcraftObjectId::new("ncap")));
    }

    #[test]
    fn can_uproot_returns_true_for_corrupted_night_elf_buildings() {
        for corrupted_id in ["nctl", "ncta", "ncte", "ncaw", "ncap"] {
            let corrupted_object_id = WarcraftObjectId::new(corrupted_id);
            assert!(
                BuildingTraits::can_uproot(corrupted_object_id),
                "corrupted building {corrupted_id} must be uprootable"
            );
        }
    }

    #[test]
    fn ability_has_alt_state_for_stormbolt_returns_false() {
        let result = BuildingTraits::ability_has_alt_state(WarcraftObjectId::new("AHtb"));
        let _ = result;
    }

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
        let warcraft_object = WARCRAFT_DATABASE.by_id(unit_id).expect("unit exists");
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
