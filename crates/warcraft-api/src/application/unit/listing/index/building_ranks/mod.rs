//! Per-building availability rank: `chain_base_gold_cost * 1000 + upgrade_depth`.
//! A main-building chain (Town Hall → Keep → Castle) shares its base's cost, so
//! the members group together and order by depth; whole chains order by base
//! cost. The `ranks` computation is pure over two plain maps; the boundary
//! extracts those maps from the database and memoizes the result.

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::WarcraftObjectMeta;
use crate::domain::unit::UnitKind;
use crate::infrastructure::database::WarcraftDatabase;

/// The upgrade-chain walk is capped so a malformed cyclic `researches` graph can
/// never loop forever.
const MAX_CHAIN_DEPTH: u32 = 16;

/// Rank every building given its gold cost and the "upgrades from" links
/// (`upgrade_parent[target] = source`). Pure over its two maps.
pub(crate) fn ranks(
    gold_cost: &HashMap<WarcraftObjectId, u32>,
    upgrade_parent: &HashMap<WarcraftObjectId, WarcraftObjectId>,
) -> HashMap<WarcraftObjectId, u32> {
    gold_cost
        .keys()
        .map(|building| {
            let mut base = *building;
            let mut depth: u32 = 0;
            while let Some(parent) = upgrade_parent.get(&base).copied() {
                base = parent;
                depth += 1;
                if depth > MAX_CHAIN_DEPTH {
                    break;
                }
            }
            let base_cost = gold_cost.get(&base).copied().unwrap_or(0);
            let key = base_cost.saturating_mul(1000).saturating_add(depth);
            (*building, key)
        })
        .collect()
}

/// The memoized per-building rank map, extracted from the database.
pub(crate) fn building_ranks(
    database: &'static WarcraftDatabase,
) -> &'static HashMap<WarcraftObjectId, u32> {
    static RANKS: OnceLock<HashMap<WarcraftObjectId, u32>> = OnceLock::new();
    RANKS.get_or_init(|| {
        let mut gold_cost: HashMap<WarcraftObjectId, u32> = HashMap::new();
        let mut upgrade_parent: HashMap<WarcraftObjectId, WarcraftObjectId> = HashMap::new();
        for (object_id, object) in database.iter() {
            let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
                continue;
            };
            if unit_meta.unit_kind() != UnitKind::Building {
                continue;
            }
            gold_cost.insert(*object_id, unit_meta.gold_cost());
            for research_id in unit_meta.researches() {
                if is_building(database, *research_id) {
                    upgrade_parent.insert(*research_id, *object_id);
                }
            }
        }
        ranks(&gold_cost, &upgrade_parent)
    })
}

/// Whether an id resolves to a building unit (research/tech ids do not).
fn is_building(database: &WarcraftDatabase, id: WarcraftObjectId) -> bool {
    database.object(id).is_some_and(|object| {
        matches!(
            object.meta(),
            WarcraftObjectMeta::Unit(unit_meta) if unit_meta.unit_kind() == UnitKind::Building
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &'static str) -> WarcraftObjectId {
        WarcraftObjectId::new(value)
    }

    #[test]
    fn a_chain_shares_its_base_cost_and_orders_by_depth() {
        let gold_cost = HashMap::from([(id("th"), 100), (id("keep"), 200), (id("cas"), 300)]);
        // keep upgrades from th; cas upgrades from keep.
        let upgrade_parent = HashMap::from([(id("keep"), id("th")), (id("cas"), id("keep"))]);

        let ranks = ranks(&gold_cost, &upgrade_parent);

        // All three share the base (th) cost 100 → 100000, ordered by depth.
        assert_eq!(ranks[&id("th")], 100_000);
        assert_eq!(ranks[&id("keep")], 100_001);
        assert_eq!(ranks[&id("cas")], 100_002);
    }

    #[test]
    fn an_independent_building_ranks_by_its_own_cost() {
        let gold_cost = HashMap::from([(id("farm"), 80)]);
        let ranks = ranks(&gold_cost, &HashMap::new());
        assert_eq!(ranks[&id("farm")], 80_000);
    }
}
