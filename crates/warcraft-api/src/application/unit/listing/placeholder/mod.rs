//! Placeholder detection: units that carry nothing to rebind and are dropped
//! from curated browsing. Two kinds — a "dead" placeholder (no production, no
//! button-positioned ability, not sold anywhere) and a "rally-only" building
//! (its only command-card button is the rally point). Both are revealed only by
//! `include_abilityless`.

use std::collections::HashSet;

use crate::application::unit::command_card::command_ids;
use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::WarcraftObjectMeta;
use crate::domain::unit::{UnitKind, UnitMeta};
use crate::infrastructure::database::WarcraftDatabase;

/// Whether the unit is a placeholder (dead or rally-only) — dropped from curated
/// browsing unless ability-less units are shown.
pub(crate) fn is_placeholder(
    database: &'static WarcraftDatabase,
    object_id: WarcraftObjectId,
    unit_meta: &UnitMeta,
    sold_units: &HashSet<WarcraftObjectId>,
) -> bool {
    is_dead_placeholder(database, object_id, unit_meta, sold_units)
        || is_rally_only_building(database, object_id, unit_meta)
}

/// A unit with no production, no button-positioned ability, and no shop slot —
/// nothing to rebind. Purchasable units (sold somewhere) are kept for their
/// buy-button hotkey.
fn is_dead_placeholder(
    database: &WarcraftDatabase,
    object_id: WarcraftObjectId,
    unit_meta: &UnitMeta,
    sold_units: &HashSet<WarcraftObjectId>,
) -> bool {
    let has_production = !unit_meta.trains().is_empty()
        || !unit_meta.builds().is_empty()
        || !unit_meta.researches().is_empty()
        || !unit_meta.sell_items().is_empty()
        || !unit_meta.sell_units().is_empty();
    let is_purchasable = sold_units.contains(&object_id);
    !has_production && !has_visible_ability(database, unit_meta) && !is_purchasable
}

/// Whether the unit carries at least one ability that occupies a command-card
/// button.
fn has_visible_ability(database: &WarcraftDatabase, unit_meta: &UnitMeta) -> bool {
    unit_meta
        .abilities()
        .iter()
        .chain(unit_meta.hero_abilities().iter())
        .any(|ability_id| {
            database.object(*ability_id).is_some_and(|ability| {
                matches!(
                    ability.meta(),
                    WarcraftObjectMeta::Ability(ability_meta)
                        if ability_meta.default_button_position().is_some()
                )
            })
        })
}

/// A building whose command card carries no ability of its own and whose only
/// command is the rally point (Demon Gate, the dimensional gates, …). The
/// per-unit command scan is gated on building kind first to keep it off the
/// common path.
fn is_rally_only_building(
    database: &'static WarcraftDatabase,
    object_id: WarcraftObjectId,
    unit_meta: &UnitMeta,
) -> bool {
    if unit_meta.unit_kind() != UnitKind::Building {
        return false;
    }
    let has_own_ability =
        !unit_meta.abilities().is_empty() || !unit_meta.hero_abilities().is_empty();
    if has_own_ability {
        return false;
    }
    command_ids(database, object_id).as_slice() == [WarcraftObjectId::new("CmdRally")]
}
