//! Which buildings can attack. A hand-maintained id list (there is no reliable
//! data-driven signal for "this building has a defensive weapon"), exposed as a
//! pure predicate over an id.

use crate::domain::identity::WarcraftObjectId;

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

/// Whether the building with this id carries a defensive weapon (guard/cannon
/// towers, attacking ziggurats, ancient protectors, …).
pub(crate) fn is_attacking_building(id: WarcraftObjectId) -> bool {
    ATTACKING_BUILDING_IDS.contains(&id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &'static str) -> WarcraftObjectId {
        WarcraftObjectId::new(value)
    }

    #[test]
    fn guard_tower_attacks() {
        assert!(is_attacking_building(id("hgtw")));
    }

    #[test]
    fn attacking_ziggurat_attacks() {
        assert!(is_attacking_building(id("unp1")));
    }

    #[test]
    fn town_hall_does_not_attack() {
        assert!(!is_attacking_building(id("htow")));
    }
}
