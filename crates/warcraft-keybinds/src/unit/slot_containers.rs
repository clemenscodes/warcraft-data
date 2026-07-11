//! The four command-card containers a unit can present (command card, build
//! menu, uprooted menu, research menu) plus its train-unit upgrade map, resolved
//! once from the database. The renderer used to orchestrate these five
//! [`UnitCommandSlots`] queries and shape the result itself at render time; that
//! is domain work (ARCHITECTURE R3), so it lives here behind one call.

use crate::identity::slot::GridSlotId;
use crate::unit::slots::UnitCommandSlots;
use std::collections::HashMap;
use std::rc::Rc;
use warcraft_api::WarcraftApi;
use warcraft_api::WarcraftObjectId;

/// A unit's resolved command containers and its train-unit upgrade map. Cheap to
/// clone (the slot lists are reference-counted), so a renderer can memoise it on
/// the selected unit id.
#[derive(Clone, Debug, PartialEq)]
pub struct UnitSlotContainers {
    command_card: Rc<[GridSlotId]>,
    build_menu: Option<Rc<[GridSlotId]>>,
    uprooted: Option<Rc<[GridSlotId]>>,
    research: Option<Rc<[GridSlotId]>>,
    train_upgrades: HashMap<WarcraftObjectId, WarcraftObjectId>,
}

impl UnitSlotContainers {
    /// Resolve every container and the upgrade map for the unit with the given id.
    /// An unknown id resolves to the default object, whose command card is empty.
    pub fn resolve(unit_id: WarcraftObjectId) -> Self {
        let unit_object_id = unit_id;
        let command_card: Rc<[GridSlotId]> = WarcraftApi::default()
            .command_card(unit_object_id)
            .filled_slots()
            .collect();
        let build_menu: Option<Rc<[GridSlotId]>> = WarcraftApi::default()
            .build_menu(unit_object_id)
            .map(|card| card.filled_slots().collect());
        let uprooted: Option<Rc<[GridSlotId]>> = WarcraftApi::default()
            .uprooted_menu(unit_object_id)
            .map(|card| card.filled_slots().collect());
        let research: Option<Rc<[GridSlotId]>> = WarcraftApi::default()
            .research_menu(unit_object_id)
            .map(|card| card.filled_slots().collect());
        let train_upgrades = WarcraftApi::default().train_unit_upgrades(unit_object_id);
        Self {
            command_card,
            build_menu,
            uprooted,
            research,
            train_upgrades,
        }
    }

    pub fn command_card(&self) -> Rc<[GridSlotId]> {
        self.command_card.clone()
    }

    pub fn build_menu(&self) -> Option<Rc<[GridSlotId]>> {
        self.build_menu.clone()
    }

    /// Whether the given slot belongs to this unit's build menu, matched
    /// case-insensitively. Which container an inspected ability edits against is a
    /// domain membership fact — the renderer must not re-derive it by scanning the
    /// slot lists itself (ARCHITECTURE R3).
    pub fn build_menu_contains(&self, slot: &GridSlotId) -> bool {
        let slot_id = slot.as_str();
        self.build_menu.as_ref().is_some_and(|list| {
            list.iter()
                .any(|candidate| candidate.as_str().eq_ignore_ascii_case(slot_id))
        })
    }

    pub fn uprooted(&self) -> Option<Rc<[GridSlotId]>> {
        self.uprooted.clone()
    }

    pub fn research(&self) -> Option<Rc<[GridSlotId]>> {
        self.research.clone()
    }

    pub fn train_upgrades(&self) -> &HashMap<WarcraftObjectId, WarcraftObjectId> {
        &self.train_upgrades
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn footman_has_a_non_empty_command_card() {
        let containers = UnitSlotContainers::resolve(crate::test_support::object_id("hfoo"));
        assert!(
            !containers.command_card().is_empty(),
            "the footman (hfoo) should have a populated command card"
        );
    }

    #[test]
    fn unknown_unit_resolves_to_empty_command_card() {
        let containers = UnitSlotContainers::resolve(crate::test_support::object_id("AHhb"));
        assert!(containers.command_card().is_empty());
        assert!(containers.build_menu().is_none());
    }

    #[test]
    fn build_menu_contains_matches_its_own_slots() {
        let containers = UnitSlotContainers::resolve(crate::test_support::object_id("hpea"));
        let build_menu = containers
            .build_menu()
            .expect("the peasant (hpea) has a build menu");
        assert!(!build_menu.is_empty(), "the build menu should be populated");
        for slot in build_menu.iter() {
            assert!(
                containers.build_menu_contains(slot),
                "every build-menu slot must report as contained"
            );
        }
    }

    #[test]
    fn build_menu_contains_is_false_without_a_build_menu() {
        let containers = UnitSlotContainers::resolve(crate::test_support::object_id("hfoo"));
        assert!(containers.build_menu().is_none());
        let command_card = containers.command_card();
        let slot = command_card
            .first()
            .expect("the footman has a command card");
        assert!(!containers.build_menu_contains(slot));
    }
}
