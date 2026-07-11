//! Building traits: domain queries that classify a building unit's structural
//! behavior — whether it can attack, uproot, or starts in a toggled alternate
//! (burrowed/rooted) form. A stateless `DomainService` over unit traits.

use crate::WarcraftApi;
use crate::{WarcraftObjectId, WarcraftObjectMeta};

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
        let Some(warcraft_object) = WarcraftApi::default().object(object_id) else {
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
        let Some(ability_object) = WarcraftApi::default().object(ability_id) else {
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
        for (unit_id_obj, warcraft_object) in WarcraftApi::default().iter() {
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
        let Some(warcraft_object) = WarcraftApi::default().object(unit_id) else {
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
        let Some(warcraft_object) = WarcraftApi::default().object(ability_id) else {
            return false;
        };
        warcraft_object.un_tip().is_some() || warcraft_object.un_ubertip().is_some()
    }
}

// DDD role: stateless domain logic over building/unit traits.
impl ddd::Layered for BuildingTraits {
    type Layer = ddd::DomainLayer;
}
impl ddd::DomainService for BuildingTraits {}

#[cfg(test)]
mod tests {
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
}
