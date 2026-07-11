use crate::GridCoordinate;
use crate::identity::slot::{CommandCard, GridSlotId};
use crate::unit::ability_rules::{
    AbilityInDatabase, AbilityOnUnit, FormUpgradeSwap, HiddenAbility, MorphAgainstHost,
    RevertsToHost, RootedOnlyAbility, UnitPair,
};
use crate::unit::alt_state::AltState;
use crate::unit::menu_commands::MenuCommands;
use ddd::Specification;
use std::collections::HashMap;
use warcraft_api::{DEVOUR_ABILITY_ID, WarcraftApi};
use warcraft_api::{UnitKind, WarcraftObjectId, WarcraftObjectMeta};

pub trait UnitCommandSlots {
    fn command_card(&self, unit_id: WarcraftObjectId) -> CommandCard;

    fn build_menu(&self, unit_id: WarcraftObjectId) -> Option<CommandCard>;

    fn research_menu(&self, unit_id: WarcraftObjectId) -> Option<CommandCard>;

    fn uprooted_menu(&self, unit_id: WarcraftObjectId) -> Option<CommandCard>;

    fn train_unit_upgrades(
        &self,
        unit_id: WarcraftObjectId,
    ) -> HashMap<WarcraftObjectId, WarcraftObjectId>;

    fn all_unit_ids(&self) -> impl Iterator<Item = WarcraftObjectId>;
}

fn slot_position(api: WarcraftApi, object_id: WarcraftObjectId) -> Option<GridCoordinate> {
    api.object(object_id)?.default_button_position()
}

fn research_slot_position(api: WarcraftApi, object_id: WarcraftObjectId) -> Option<GridCoordinate> {
    api.object(object_id)?.default_research_button_position()
}

impl UnitCommandSlots for WarcraftApi {
    fn command_card(&self, unit_id: WarcraftObjectId) -> CommandCard {
        let Some(unit_object) = self.object(unit_id) else {
            return CommandCard::empty();
        };
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return CommandCard::empty();
        };
        let primary_commands: Vec<WarcraftObjectId> = self
            .unit()
            .command_card(unit_id)
            .iter()
            .map(|command| command.id())
            .collect();
        let unit_kind = unit_meta.effective_kind();
        let regular_abilities = unit_meta.abilities();
        let hero_abilities = unit_meta.hero_abilities();
        let primary_train_slots = if unit_kind == UnitKind::Building {
            unit_meta.trains()
        } else {
            &[]
        };
        let primary_research_slots = if unit_kind == UnitKind::Building {
            unit_meta.researches()
        } else {
            &[]
        };
        let sell_items = if unit_kind == UnitKind::Building {
            unit_meta.sell_items()
        } else {
            &[]
        };
        let sell_units = if unit_kind == UnitKind::Building {
            unit_meta.sell_units()
        } else {
            &[]
        };
        let mut card = CommandCard::empty();
        for command_name in primary_commands {
            let command_object = self.object(command_name);
            let command_has_icon =
                command_object.is_some_and(|object| object.has_displayable_icon());
            if !command_has_icon {
                continue;
            }
            let Some(slot_position) = slot_position(*self, command_name) else {
                continue;
            };
            let command_slot = GridSlotId::command(command_name);
            card.place(slot_position, command_slot);
        }
        let mut unplaced_train_slots: Vec<GridSlotId> = Vec::new();
        for trained_id in primary_train_slots {
            let trained_object_id = *trained_id;
            let trained_object = self.object(trained_object_id);
            let trained_has_icon =
                trained_object.is_some_and(|object| object.has_displayable_icon());
            if !trained_has_icon {
                continue;
            }
            let train_slot = GridSlotId::ability(trained_object_id);
            match slot_position(*self, trained_object_id) {
                Some(slot_position) => {
                    if !card.place(slot_position, train_slot) {
                        let occupant = card.slot_at(slot_position);
                        let collapses_into_swap = occupant.is_some_and(|existing| {
                            let existing_id = existing.id();
                            let pair = UnitPair::new(trained_object_id, existing_id);
                            FormUpgradeSwap.is_satisfied_by(&pair)
                        });
                        if !collapses_into_swap {
                            unplaced_train_slots.push(train_slot);
                        }
                    }
                }
                None => {
                    unplaced_train_slots.push(train_slot);
                }
            }
        }
        for unplaced_slot in unplaced_train_slots {
            card.place_at_next_empty(unplaced_slot);
        }
        let mut unplaced_research_slots: Vec<GridSlotId> = Vec::new();
        for research_id in primary_research_slots {
            let research_object_id = *research_id;
            let research_object = self.object(research_object_id);
            let research_has_icon =
                research_object.is_some_and(|object| object.has_displayable_icon());
            if !research_has_icon {
                continue;
            }
            let research_slot = GridSlotId::ability(research_object_id);
            match slot_position(*self, research_object_id) {
                Some(slot_position) => {
                    if !card.place(slot_position, research_slot) {
                        unplaced_research_slots.push(research_slot);
                    }
                }
                None => {
                    unplaced_research_slots.push(research_slot);
                }
            }
        }
        for unplaced_slot in unplaced_research_slots {
            card.place_at_next_empty(unplaced_slot);
        }
        let mut unplaced_sell_item_slots: Vec<GridSlotId> = Vec::new();
        for sell_item_id in sell_items {
            let sell_item_object_id = *sell_item_id;
            let sell_item_object = self.object(sell_item_object_id);
            let sell_item_has_icon =
                sell_item_object.is_some_and(|object| object.has_displayable_icon());
            if !sell_item_has_icon {
                continue;
            }
            let sell_item_slot = GridSlotId::ability(sell_item_object_id);
            match slot_position(*self, sell_item_object_id) {
                Some(sell_item_position) => {
                    if !card.place(sell_item_position, sell_item_slot) {
                        unplaced_sell_item_slots.push(sell_item_slot);
                    }
                }
                None => {
                    unplaced_sell_item_slots.push(sell_item_slot);
                }
            }
        }
        for unplaced_slot in unplaced_sell_item_slots {
            card.place_at_next_empty(unplaced_slot);
        }
        let mut unplaced_sell_unit_slots: Vec<GridSlotId> = Vec::new();
        for sell_unit_id in sell_units {
            let sell_unit_object_id = *sell_unit_id;
            let sell_unit_object = self.object(sell_unit_object_id);
            let sell_unit_has_icon =
                sell_unit_object.is_some_and(|object| object.has_displayable_icon());
            if !sell_unit_has_icon {
                continue;
            }
            let sell_unit_slot = GridSlotId::ability(sell_unit_object_id);
            match slot_position(*self, sell_unit_object_id) {
                Some(sell_unit_position) => {
                    if !card.place(sell_unit_position, sell_unit_slot) {
                        unplaced_sell_unit_slots.push(sell_unit_slot);
                    }
                }
                None => {
                    unplaced_sell_unit_slots.push(sell_unit_slot);
                }
            }
        }
        for unplaced_slot in unplaced_sell_unit_slots {
            card.place_at_next_empty(unplaced_slot);
        }
        let is_uprootable = self.unit().can_uproot(unit_id);
        let host_is_burrowed = self.is_burrowed_form(unit_id);
        let host_is_in_alt_state = self.unit_starts_in_toggle_alt_state(unit_id);
        let devour_ability_id = DEVOUR_ABILITY_ID;
        let mut occupied_on_positions: Vec<GridCoordinate> = Vec::new();
        for ability_id in regular_abilities.iter().chain(hero_abilities.iter()) {
            if let Some(on_position) = slot_position(*self, *ability_id) {
                occupied_on_positions.push(on_position);
            }
        }
        let mut unplaced_ability_slots: Vec<GridSlotId> = Vec::new();
        for ability_id in regular_abilities.iter().chain(hero_abilities.iter()) {
            let ability_object_id = *ability_id;
            if hero_abilities.contains(ability_id) {
                let levelable_object = self.object(ability_object_id);
                let is_levelable = levelable_object
                    .map(|object| match object.meta() {
                        WarcraftObjectMeta::Ability(meta) => {
                            meta.max_level() > 1 || meta.is_ultimate()
                        }
                        _ => true,
                    })
                    .unwrap_or(true);
                if !is_levelable {
                    continue;
                }
            }
            let hidden_candidate = AbilityOnUnit::new(unit_id, ability_object_id);
            if HiddenAbility.is_satisfied_by(&hidden_candidate) {
                continue;
            }
            if is_uprootable && ability_object_id == devour_ability_id {
                continue;
            }
            if host_is_burrowed && !self.ability_has_alt_state(ability_object_id) {
                continue;
            }
            let morph_candidate = MorphAgainstHost::new(*self, ability_object_id, unit_id);
            if RevertsToHost.is_satisfied_by(&morph_candidate) {
                continue;
            }
            let ability_database_object = self.object(ability_object_id);
            let ability_has_icon =
                ability_database_object.is_some_and(|object| object.has_displayable_icon());
            if !ability_has_icon {
                continue;
            }
            let morph_target_object = self.object(ability_object_id);
            let morph_target_id =
                morph_target_object.and_then(|object| object.ability_morph_target_id());
            let is_morph_back = morph_target_id.is_some_and(|target| target == unit_id);
            let use_off_state = is_morph_back
                || (host_is_in_alt_state && self.ability_has_alt_state(ability_object_id));
            let ability_slot = if use_off_state {
                GridSlotId::ability_off(ability_object_id)
            } else {
                GridSlotId::ability(ability_object_id)
            };
            let off_state_object = self.object(ability_object_id);
            let off_state_position = off_state_object.and_then(|object| match object.meta() {
                WarcraftObjectMeta::Ability(ability_meta) => ability_meta.off_button_position(),
                _ => None,
            });
            match slot_position(*self, ability_object_id) {
                Some(ability_position) => {
                    if !card.place(ability_position, ability_slot) {
                        unplaced_ability_slots.push(ability_slot);
                    }
                    if !use_off_state
                        && unit_kind == UnitKind::Building
                        && let Some(off_position) = off_state_position
                        && off_position != ability_position
                        && !occupied_on_positions.contains(&off_position)
                    {
                        let off_state_slot = GridSlotId::ability_off(ability_object_id);
                        if !card.place(off_position, off_state_slot) {
                            unplaced_ability_slots.push(off_state_slot);
                        }
                    }
                }
                None => {
                    unplaced_ability_slots.push(ability_slot);
                }
            }
        }
        for unplaced_slot in unplaced_ability_slots {
            card.place_at_next_empty(unplaced_slot);
        }
        if unit_kind == UnitKind::Hero
            && !hero_abilities.is_empty()
            && let Some(select_skill) = self.select_skill_command()
        {
            let select_skill_object = self.object(select_skill);
            let select_skill_has_icon =
                select_skill_object.is_some_and(|object| object.has_displayable_icon());
            if select_skill_has_icon {
                let position_option = slot_position(*self, select_skill);
                if let Some(slot_position) = position_option {
                    let select_skill_slot = GridSlotId::command(select_skill);
                    card.place(slot_position, select_skill_slot);
                }
            }
        }
        card
    }

    fn build_menu(&self, unit_id: WarcraftObjectId) -> Option<CommandCard> {
        let unit_object = self.object(unit_id)?;
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return None;
        };
        if unit_meta.effective_kind() != UnitKind::Worker {
            return None;
        }
        if unit_meta.builds().is_empty() {
            return None;
        }
        let build_menu_commands = self.build_menu_commands(unit_meta);
        let mut card = CommandCard::empty();
        for command_name in build_menu_commands {
            let command_object = self.object(command_name);
            let command_has_icon =
                command_object.is_some_and(|object| object.has_displayable_icon());
            if !command_has_icon {
                continue;
            }
            let Some(slot_position) = slot_position(*self, command_name) else {
                continue;
            };
            let command_slot = GridSlotId::command(command_name);
            card.place(slot_position, command_slot);
        }
        for production_id in unit_meta.builds() {
            let production_object_id = *production_id;
            let production_object = self.object(production_object_id);
            let production_has_icon =
                production_object.is_some_and(|object| object.has_displayable_icon());
            if !production_has_icon {
                continue;
            }
            let Some(slot_position) = slot_position(*self, production_object_id) else {
                continue;
            };
            let production_slot = GridSlotId::ability(production_object_id);
            card.place(slot_position, production_slot);
        }
        Some(card)
    }

    fn research_menu(&self, unit_id: WarcraftObjectId) -> Option<CommandCard> {
        let unit_object = self.object(unit_id)?;
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return None;
        };
        if unit_meta.effective_kind() != UnitKind::Hero {
            return None;
        }
        let hero_abilities = unit_meta.hero_abilities();
        if hero_abilities.is_empty() {
            return None;
        }
        let mut card = CommandCard::empty();
        for ability_id in hero_abilities.iter() {
            let ability_object_id = *ability_id;
            let ability_object = self.object(ability_object_id);
            let ability_has_icon =
                ability_object.is_some_and(|object| object.has_displayable_icon());
            if !ability_has_icon {
                continue;
            }
            let Some(slot_position) = research_slot_position(*self, ability_object_id) else {
                continue;
            };
            let ability_slot = GridSlotId::ability(ability_object_id);
            card.place(slot_position, ability_slot);
        }
        if let Some(back_command) = self.submenu_back_command() {
            let back_command_object = self.object(back_command);
            let back_command_has_icon =
                back_command_object.is_some_and(|object| object.has_displayable_icon());
            if back_command_has_icon {
                let position_option = slot_position(*self, back_command);
                if let Some(slot_position) = position_option {
                    let back_slot = GridSlotId::command(back_command);
                    card.place(slot_position, back_slot);
                }
            }
        }
        Some(card)
    }

    fn uprooted_menu(&self, unit_id: WarcraftObjectId) -> Option<CommandCard> {
        let unit_object = self.object(unit_id)?;
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return None;
        };
        if unit_meta.effective_kind() != UnitKind::Building {
            return None;
        }
        if !self.unit().can_uproot(unit_id) {
            return None;
        }
        let mut card = CommandCard::empty();
        for command_name in self.mobile_command_ids().iter().copied() {
            let command_object = self.object(command_name);
            let command_has_icon =
                command_object.is_some_and(|object| object.has_displayable_icon());
            if !command_has_icon {
                continue;
            }
            let Some(slot_position) = slot_position(*self, command_name) else {
                continue;
            };
            let command_slot = GridSlotId::command(command_name);
            card.place(slot_position, command_slot);
        }
        for ability_id in unit_meta.abilities() {
            let ability_object_id = *ability_id;
            let ability_object = self.object(ability_object_id);
            let ability_has_icon =
                ability_object.is_some_and(|object| object.has_displayable_icon());
            if !ability_has_icon {
                continue;
            }
            let morph_candidate = MorphAgainstHost::new(*self, ability_object_id, unit_id);
            if RevertsToHost.is_satisfied_by(&morph_candidate) {
                continue;
            }
            let rooted_candidate = AbilityInDatabase::new(*self, ability_object_id);
            if RootedOnlyAbility.is_satisfied_by(&rooted_candidate) {
                continue;
            }
            let Some(slot_position) = slot_position(*self, ability_object_id) else {
                continue;
            };
            let ability_slot = GridSlotId::ability(ability_object_id);
            card.place(slot_position, ability_slot);
        }
        Some(card)
    }

    fn train_unit_upgrades(
        &self,
        unit_id: WarcraftObjectId,
    ) -> HashMap<WarcraftObjectId, WarcraftObjectId> {
        let Some(unit_object) = self.object(unit_id) else {
            return HashMap::new();
        };
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return HashMap::new();
        };
        let primary_train_slots = unit_meta.trains();
        let mut seen_positions: HashMap<crate::GridCoordinate, WarcraftObjectId> = HashMap::new();
        let mut upgrades: HashMap<WarcraftObjectId, WarcraftObjectId> = HashMap::new();
        for trained_id in primary_train_slots {
            let trained_object_id = *trained_id;
            let trained_object = self.object(trained_object_id);
            let has_icon = trained_object.is_some_and(|object| object.has_displayable_icon());
            if !has_icon {
                continue;
            }
            let position_option =
                trained_object.and_then(|object| object.default_button_position());
            let Some(position) = position_option else {
                continue;
            };
            if let Some(existing_id) = seen_positions.get(&position).copied() {
                let pair = UnitPair::new(existing_id, trained_object_id);
                if FormUpgradeSwap.is_satisfied_by(&pair) {
                    upgrades.entry(existing_id).or_insert(trained_object_id);
                }
            } else {
                seen_positions.insert(position, trained_object_id);
            }
        }
        upgrades
    }

    fn all_unit_ids(&self) -> impl Iterator<Item = WarcraftObjectId> {
        self.iter().filter_map(|(database_id, warcraft_object)| {
            if matches!(warcraft_object.meta(), WarcraftObjectMeta::Unit(_)) {
                Some(*database_id)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests;
