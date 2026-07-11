//! Unit facts: the minimal slice of the database the variant build needs,
//! extracted once. [`extract`] is the *only* function in the whole subsystem
//! that reads the database; everything downstream receives these plain id sets,
//! never the database, and each downstream function takes only the set it uses.

use std::collections::{BTreeMap, HashSet};

use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::WarcraftObjectMeta;
use crate::domain::unit::UnitKind;
use crate::infrastructure::database::WarcraftDatabase;

/// The database-derived inputs to variant grouping — exactly what the pure chain
/// builders consume, nothing more.
pub(crate) struct UnitFacts {
    /// Ids that are real, non-hero units (the "mergeable into a tier" set).
    pub(crate) non_hero_units: HashSet<WarcraftObjectId>,
    /// Hero unit ids grouped by shared display name, in deterministic name order.
    pub(crate) heroes_by_name: BTreeMap<&'static str, Vec<WarcraftObjectId>>,
    /// Ids that some unit trains or sells — the "produced" (real, playable)
    /// signal used to pick a hero group's canonical.
    pub(crate) produced_units: HashSet<WarcraftObjectId>,
}

/// Scan the database once into the minimal facts the variant build needs.
pub(crate) fn extract(database: &WarcraftDatabase) -> UnitFacts {
    let mut non_hero_units: HashSet<WarcraftObjectId> = HashSet::new();
    let mut heroes_by_name: BTreeMap<&'static str, Vec<WarcraftObjectId>> = BTreeMap::new();
    let mut produced_units: HashSet<WarcraftObjectId> = HashSet::new();

    for (object_id, object) in database.iter() {
        let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
            continue;
        };
        for trained_id in unit_meta.trains() {
            produced_units.insert(*trained_id);
        }
        for sold_id in unit_meta.sell_units() {
            produced_units.insert(*sold_id);
        }
        if unit_meta.unit_kind() == UnitKind::Hero {
            if let Some(display_name) = object.names().first().copied()
                && !display_name.is_empty()
            {
                heroes_by_name
                    .entry(display_name)
                    .or_default()
                    .push(*object_id);
            }
        } else {
            non_hero_units.insert(*object_id);
        }
    }

    UnitFacts {
        non_hero_units,
        heroes_by_name,
        produced_units,
    }
}
