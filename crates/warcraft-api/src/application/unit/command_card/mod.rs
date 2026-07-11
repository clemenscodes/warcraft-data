//! Command-card assembly for a unit: the ordered command buttons it shows.
//! `assembly` is the pure core; this module is the boundary that extracts a
//! unit's context and the known-command set from the database and resolves the
//! resulting ids to [`CommandView`]s. The database is threaded in.

pub(crate) mod assembly;

use std::collections::HashSet;
use std::sync::OnceLock;

use crate::application::view::command::CommandView;
use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::{WarcraftObjectKind, WarcraftObjectMeta};
use crate::domain::unit::attacking_buildings::is_attacking_building;
use crate::infrastructure::database::WarcraftDatabase;
use assembly::{CommandContext, command_card_ids};

/// The ordered command-button ids the unit with this id shows. Empty when the id
/// is unknown or names a non-unit. Used both by the public `command_card` edge
/// and by the catalog's rally-only placeholder rule.
pub(crate) fn command_ids(
    database: &'static WarcraftDatabase,
    unit_id: WarcraftObjectId,
) -> Vec<WarcraftObjectId> {
    let Some(object) = database.object(unit_id) else {
        return Vec::new();
    };
    let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
        return Vec::new();
    };
    let has_builds = !unit_meta.builds().is_empty();
    let context = CommandContext {
        kind: unit_meta.effective_kind(),
        has_production: has_builds || !unit_meta.trains().is_empty(),
        has_builds,
        can_attack: is_attacking_building(unit_id),
        targets_ground: unit_meta
            .combat()
            .attack()
            .is_some_and(|attack| attack.targets_ground()),
        race: object.race(),
    };
    command_card_ids(&context, known_commands(database))
}

/// The unit's command-card buttons as resolved [`CommandView`]s.
pub(crate) fn command_card(
    database: &'static WarcraftDatabase,
    unit_id: WarcraftObjectId,
) -> Vec<CommandView> {
    command_ids(database, unit_id)
        .into_iter()
        .filter_map(|command_id| {
            database
                .object(command_id)
                .and_then(|object| CommandView::try_from(object).ok())
        })
        .collect()
}

/// Every command id that exists in the database, built once and memoized.
fn known_commands(database: &'static WarcraftDatabase) -> &'static HashSet<WarcraftObjectId> {
    static KNOWN: OnceLock<HashSet<WarcraftObjectId>> = OnceLock::new();
    KNOWN.get_or_init(|| {
        database
            .iter()
            .filter(|(_id, object)| object.kind() == WarcraftObjectKind::Command)
            .map(|(id, _object)| *id)
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use crate::WarcraftApi;
    use crate::domain::identity::WarcraftObjectId;

    fn shows_attack_ground(unit: &'static str) -> bool {
        let attack_ground = WarcraftObjectId::new("CmdAttackGround");
        WarcraftApi::default()
            .unit()
            .command_card(WarcraftObjectId::new(unit))
            .iter()
            .any(|command| command.id() == attack_ground)
    }

    #[test]
    fn artillery_units_show_attack_ground() {
        assert!(shows_attack_ground("hmtm"), "Mortar Team");
        assert!(shows_attack_ground("ocat"), "Demolisher");
    }

    #[test]
    fn normal_weapon_units_and_buildings_omit_attack_ground() {
        assert!(!shows_attack_ground("hfoo"), "Footman");
        assert!(!shows_attack_ground("hctw"), "Cannon Tower");
    }
}
