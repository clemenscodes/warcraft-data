//! Pure command-card assembly: given a unit's command-relevant context and the
//! set of command ids that actually exist in the database, produce the ordered
//! list of command-button ids the unit shows. No database, no globals — the
//! context and the known-command set are passed in, so every branch is tested in
//! isolation. Command-id literals are compile-time constants, not global state.

use std::collections::HashSet;

use crate::domain::identity::WarcraftObjectId;
use crate::domain::race::Race;
use crate::domain::unit::UnitKind;

const CMD_ATTACK: WarcraftObjectId = WarcraftObjectId::new("CmdAttack");
const CMD_MOVE: WarcraftObjectId = WarcraftObjectId::new("CmdMove");
const CMD_STOP: WarcraftObjectId = WarcraftObjectId::new("CmdStop");
const CMD_HOLD_POS: WarcraftObjectId = WarcraftObjectId::new("CmdHoldPos");
const CMD_PATROL: WarcraftObjectId = WarcraftObjectId::new("CmdPatrol");
const CMD_ATTACK_GROUND: WarcraftObjectId = WarcraftObjectId::new("CmdAttackGround");
const CMD_CANCEL_TRAIN: WarcraftObjectId = WarcraftObjectId::new("CmdCancelTrain");
const CMD_RALLY: WarcraftObjectId = WarcraftObjectId::new("CmdRally");
const CMD_BUILD: WarcraftObjectId = WarcraftObjectId::new("CmdBuild");

/// The mobile-unit base command set, in card order.
const MOBILE_COMMANDS: &[WarcraftObjectId] =
    &[CMD_ATTACK, CMD_MOVE, CMD_STOP, CMD_HOLD_POS, CMD_PATROL];

/// A building's base command set (rally / cancel-train), gated on production.
const BUILDING_COMMANDS: &[WarcraftObjectId] = &[CMD_CANCEL_TRAIN, CMD_RALLY];

/// The extra commands an attacking building (tower) shows.
const TOWER_COMMANDS: &[WarcraftObjectId] = &[CMD_ATTACK, CMD_STOP];

/// Context/cancel commands that never appear on the rendered card — stripped
/// after assembly. `CmdCancelTrain` earns its place in the building list only to
/// gate rally, then is removed here.
const CONTEXT_COMMANDS: &[WarcraftObjectId] = &[
    WarcraftObjectId::new("CmdCancel"),
    WarcraftObjectId::new("CmdCancelBuild"),
    WarcraftObjectId::new("CmdCancelRevive"),
    CMD_CANCEL_TRAIN,
];

/// The command-relevant facts about a unit — everything the pure assembly needs,
/// extracted from the unit's meta at the boundary.
pub(crate) struct CommandContext {
    /// The unit's effective kind (special workers count as soldiers).
    pub(crate) kind: UnitKind,
    /// Whether the unit builds or trains anything.
    pub(crate) has_production: bool,
    /// Whether the unit builds (a subset of production, gates the build menu).
    pub(crate) has_builds: bool,
    /// Whether the unit is an attacking building.
    pub(crate) can_attack: bool,
    /// Whether the unit's weapon targets ground (grants Attack Ground).
    pub(crate) targets_ground: bool,
    /// The unit's race (selects the race-specific build command).
    pub(crate) race: Option<Race>,
}

/// The ordered command-button ids a unit shows, filtered to commands that exist
/// in the database and with context/cancel commands stripped.
pub(crate) fn command_card_ids(
    context: &CommandContext,
    known_commands: &HashSet<WarcraftObjectId>,
) -> Vec<WarcraftObjectId> {
    let mut commands: Vec<WarcraftObjectId> = match context.kind {
        UnitKind::Building => building_commands(context),
        UnitKind::Worker | UnitKind::Soldier | UnitKind::Hero => {
            mobile_commands(context, known_commands)
        }
    };
    commands.retain(|command| known_commands.contains(command));
    commands.retain(|command| !CONTEXT_COMMANDS.contains(command));
    commands
}

/// A building's card before filtering: tower commands (when it attacks) then the
/// production-gated rally / cancel-train commands.
fn building_commands(context: &CommandContext) -> Vec<WarcraftObjectId> {
    let mut commands: Vec<WarcraftObjectId> = Vec::new();
    if context.can_attack {
        commands.extend_from_slice(TOWER_COMMANDS);
    }
    for command in BUILDING_COMMANDS.iter().copied() {
        let production_gated = command == CMD_RALLY || command == CMD_CANCEL_TRAIN;
        if production_gated && !context.has_production {
            continue;
        }
        commands.push(command);
    }
    commands
}

/// A mobile unit's card before filtering: the base set, plus Attack Ground for
/// ground-targeting weapons, plus a leading build command for building workers.
fn mobile_commands(
    context: &CommandContext,
    known_commands: &HashSet<WarcraftObjectId>,
) -> Vec<WarcraftObjectId> {
    let mut commands: Vec<WarcraftObjectId> = MOBILE_COMMANDS.to_vec();
    if context.targets_ground {
        commands.push(CMD_ATTACK_GROUND);
    }
    if context.has_builds
        && context.kind == UnitKind::Worker
        && let Some(build) = build_command(context.race, known_commands)
    {
        commands.insert(0, build);
    }
    commands
}

/// The race-specific build command, falling back to the generic `CmdBuild`.
/// `None` when neither exists in the database or the race is unknown.
fn build_command(
    race: Option<Race>,
    known_commands: &HashSet<WarcraftObjectId>,
) -> Option<WarcraftObjectId> {
    let preferred = match race? {
        Race::Human => WarcraftObjectId::new("CmdBuildHuman"),
        Race::Orc => WarcraftObjectId::new("CmdBuildOrc"),
        Race::Nightelf => WarcraftObjectId::new("CmdBuildNightElf"),
        Race::Undead => WarcraftObjectId::new("CmdBuildUndead"),
        Race::Neutral => CMD_BUILD,
    };
    if known_commands.contains(&preferred) {
        Some(preferred)
    } else if known_commands.contains(&CMD_BUILD) {
        Some(CMD_BUILD)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn known_all() -> HashSet<WarcraftObjectId> {
        HashSet::from([
            CMD_ATTACK,
            CMD_MOVE,
            CMD_STOP,
            CMD_HOLD_POS,
            CMD_PATROL,
            CMD_ATTACK_GROUND,
            CMD_CANCEL_TRAIN,
            CMD_RALLY,
            CMD_BUILD,
            WarcraftObjectId::new("CmdBuildOrc"),
        ])
    }

    fn context(kind: UnitKind) -> CommandContext {
        CommandContext {
            kind,
            has_production: false,
            has_builds: false,
            can_attack: false,
            targets_ground: false,
            race: None,
        }
    }

    #[test]
    fn a_producing_building_with_no_weapon_shows_only_rally() {
        let ctx = CommandContext {
            has_production: true,
            ..context(UnitKind::Building)
        };
        assert_eq!(command_card_ids(&ctx, &known_all()), [CMD_RALLY]);
    }

    #[test]
    fn an_attacking_producing_building_shows_tower_commands_then_rally() {
        let ctx = CommandContext {
            has_production: true,
            can_attack: true,
            ..context(UnitKind::Building)
        };
        assert_eq!(
            command_card_ids(&ctx, &known_all()),
            [CMD_ATTACK, CMD_STOP, CMD_RALLY]
        );
    }

    #[test]
    fn a_non_producing_plain_building_shows_nothing() {
        assert!(command_card_ids(&context(UnitKind::Building), &known_all()).is_empty());
    }

    #[test]
    fn a_soldier_shows_the_mobile_command_set() {
        assert_eq!(
            command_card_ids(&context(UnitKind::Soldier), &known_all()),
            [CMD_ATTACK, CMD_MOVE, CMD_STOP, CMD_HOLD_POS, CMD_PATROL]
        );
    }

    #[test]
    fn an_artillery_unit_gains_attack_ground() {
        let ctx = CommandContext {
            targets_ground: true,
            ..context(UnitKind::Soldier)
        };
        assert!(command_card_ids(&ctx, &known_all()).contains(&CMD_ATTACK_GROUND));
    }

    #[test]
    fn attack_ground_is_dropped_when_the_command_is_unknown() {
        let ctx = CommandContext {
            targets_ground: true,
            ..context(UnitKind::Soldier)
        };
        let known = HashSet::from([CMD_ATTACK, CMD_MOVE, CMD_STOP, CMD_HOLD_POS, CMD_PATROL]);
        assert!(!command_card_ids(&ctx, &known).contains(&CMD_ATTACK_GROUND));
    }

    #[test]
    fn a_building_worker_gets_its_race_build_command_first() {
        let ctx = CommandContext {
            has_builds: true,
            race: Some(Race::Orc),
            ..context(UnitKind::Worker)
        };
        let card = command_card_ids(&ctx, &known_all());
        assert_eq!(card.first(), Some(&WarcraftObjectId::new("CmdBuildOrc")));
    }
}
