//! Pure listing sort key. Within a listing, entries order by: category priority
//! (search scope sinks campaign units below their melee counterparts), then
//! in-game availability (buildings by upgrade-chain rank, everything else by
//! tech tier), then display name, then id. Tuples derive lexicographic `Ord`, so
//! the whole comparator is one key function.

use crate::domain::identity::WarcraftObjectId;
use crate::domain::unit::UnitKind;

/// The sort key for one listing entry. `building_rank` is the entry's
/// upgrade-chain rank (used only for buildings); `level` is its tech tier (used
/// for everything else). `is_search` selects the campaign-sinking priority.
pub(crate) fn sort_key(
    kind: UnitKind,
    is_campaign: bool,
    is_search: bool,
    building_rank: u32,
    level: u32,
    name: &str,
    id: WarcraftObjectId,
) -> (u8, u32, &str, WarcraftObjectId) {
    let priority = if is_search {
        kind.search_sort_priority(is_campaign)
    } else {
        kind.category_priority()
    };
    let availability = if kind == UnitKind::Building {
        building_rank
    } else {
        level
    };
    (priority, availability, name, id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &'static str) -> WarcraftObjectId {
        WarcraftObjectId::new(value)
    }

    #[test]
    fn browse_orders_by_category_priority() {
        let hero = sort_key(UnitKind::Hero, false, false, 0, 0, "h", id("h")).0;
        let building = sort_key(UnitKind::Building, false, false, 0, 0, "b", id("b")).0;
        let worker = sort_key(UnitKind::Worker, false, false, 0, 0, "w", id("w")).0;
        let soldier = sort_key(UnitKind::Soldier, false, false, 0, 0, "s", id("s")).0;
        assert!(hero < building && building < worker && worker < soldier);
    }

    #[test]
    fn search_sinks_campaign_units_below_melee() {
        let melee_soldier = sort_key(UnitKind::Soldier, false, true, 0, 0, "s", id("s")).0;
        let campaign_hero = sort_key(UnitKind::Hero, true, true, 0, 0, "h", id("h")).0;
        assert!(melee_soldier < campaign_hero);
    }

    #[test]
    fn buildings_use_the_upgrade_rank_and_others_use_the_level() {
        let building = sort_key(UnitKind::Building, false, false, 42, 7, "b", id("b")).1;
        let soldier = sort_key(UnitKind::Soldier, false, false, 42, 7, "s", id("s")).1;
        assert_eq!(building, 42, "building uses upgrade rank");
        assert_eq!(soldier, 7, "soldier uses tech tier");
    }
}
