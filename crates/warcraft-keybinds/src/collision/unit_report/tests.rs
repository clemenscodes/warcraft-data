#[cfg(test)]
mod unit_collision_report_tests {
    use super::super::*;
    use crate::model::{AbilityBinding, ColumnIndex, GridCoordinate, Hotkey, RowIndex};
    use crate::unit::grids::{GridRole, HotkeyCollisionCardBuilder, PositionCollisionCardBuilder};
    use warcraft_api::WarcraftObjectId;

    #[derive(Clone, Debug, PartialEq, Default)]
    struct UnitCollisionReportBuilder {
        entries: Vec<UnitCollisionEntry>,
    }

    impl UnitCollisionReportBuilder {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }

        fn entry(mut self, entry: UnitCollisionEntry) -> Self {
            self.entries.push(entry);
            self
        }

        fn build(self) -> UnitCollisionReport {
            UnitCollisionReport {
                entries: self.entries,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct UnitCollisionEntryBuilder {
        unit_id: WarcraftObjectId,
        unit_name: &'static str,
        position_cards: [PositionCollisionCard; 2],
        hotkey_cards: [HotkeyCollisionCard; 2],
    }

    impl UnitCollisionEntryBuilder {
        fn new(
            unit_id: &'static str,
            unit_name: &'static str,
            empty_pos: PositionCollisionCard,
            empty_hot: HotkeyCollisionCard,
        ) -> Self {
            Self {
                unit_id: crate::test_support::object_id(unit_id),
                unit_name,
                position_cards: [empty_pos, empty_pos],
                hotkey_cards: [empty_hot, empty_hot],
            }
        }

        fn main_position_card(mut self, card: PositionCollisionCard) -> Self {
            self.position_cards[0] = card;
            self
        }

        fn main_hotkey_card(mut self, card: HotkeyCollisionCard) -> Self {
            self.hotkey_cards[0] = card;
            self
        }

        fn secondary_hotkey_card(mut self, card: HotkeyCollisionCard) -> Self {
            self.hotkey_cards[1] = card;
            self
        }

        fn build(self) -> UnitCollisionEntry {
            UnitCollisionEntry {
                unit_id: self.unit_id,
                unit_name: self.unit_name,
                position_cards: self.position_cards,
                hotkey_cards: self.hotkey_cards,
            }
        }
    }

    fn paladin_id() -> WarcraftObjectId {
        crate::test_support::object_id("Hpal")
    }

    #[test]
    fn paladin_has_no_collisions_in_normalized_default() {
        let custom_keys = CustomKeys::from_text("");
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let paladin_filtered = report.for_unit(paladin_id());
        assert!(
            paladin_filtered.is_empty(),
            "Paladin must have no collisions in the normalized default on QWERTY",
        );
    }

    #[test]
    fn detects_position_collision_across_all_units() {
        let shared_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Two);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability(crate::test_support::object_id("AHhb"), holy_light_binding);
        custom_keys.put_ability(
            crate::test_support::object_id("AHds"),
            divine_shield_binding,
        );
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let paladin_entry = report
            .entries()
            .iter()
            .find(|entry| entry.unit_id() == paladin_id());
        assert!(
            paladin_entry.is_some(),
            "Paladin must appear in collision report when two abilities share a position",
        );
        let entry = paladin_entry.unwrap();
        assert!(
            entry.position_cards().iter().any(|card| !card.is_empty()),
            "position collision card must be present for Paladin",
        );
        let has_collision_at_position = entry
            .position_cards()
            .iter()
            .any(|card| card.collision_at(shared_position).is_some());
        assert!(
            has_collision_at_position,
            "collision must be reported at the shared position",
        );
    }

    #[test]
    fn detects_hotkey_collision_across_all_units() {
        let hotkey_q = Hotkey::Letter('Q');
        let first_cell = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let second_cell = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(first_cell)
            .hotkey(hotkey_q)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(second_cell)
            .hotkey(hotkey_q)
            .build();
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability(crate::test_support::object_id("AHhb"), holy_light_binding);
        custom_keys.put_ability(
            crate::test_support::object_id("AHds"),
            divine_shield_binding,
        );
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let paladin_entry = report
            .entries()
            .iter()
            .find(|entry| entry.unit_id() == paladin_id());
        assert!(
            paladin_entry.is_some(),
            "Paladin must appear in collision report when two abilities share a hotkey",
        );
        let entry = paladin_entry.unwrap();
        assert!(
            entry.hotkey_cards().iter().any(|card| !card.is_empty()),
            "hotkey collision card must be present for Paladin",
        );
    }

    #[test]
    fn for_unit_filters_to_matching_unit() {
        let shared_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Two);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability(crate::test_support::object_id("AHhb"), holy_light_binding);
        custom_keys.put_ability(
            crate::test_support::object_id("AHds"),
            divine_shield_binding,
        );
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let filtered = report.for_unit(paladin_id());
        assert!(
            filtered
                .entries()
                .iter()
                .all(|entry| entry.unit_id() == paladin_id()),
            "for_unit must return only entries for the requested unit",
        );
    }

    #[test]
    fn for_unit_returns_empty_for_unknown_unit() {
        let custom_keys = CustomKeys::from_text("");
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let filtered = report.for_unit(crate::test_support::object_id("AHhb"));
        assert!(
            filtered.is_empty(),
            "unknown unit id must yield empty report"
        );
    }

    #[test]
    fn entries_are_sorted_by_unit_name() {
        let custom_keys = CustomKeys::from_text("");
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let names: Vec<&str> = report
            .entries()
            .iter()
            .map(|entry| entry.unit_name())
            .collect();
        let mut sorted_names = names.clone();
        sorted_names.sort();
        assert_eq!(names, sorted_names, "entries must be sorted by unit name");
    }

    #[test]
    fn default_customkeys_collision_matches_expected() {
        let join_handle = std::thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .spawn(run_default_collision_check)
            .unwrap();
        join_handle.join().unwrap();
    }

    #[test]
    #[ignore = "code generator — run with --ignored to refresh the snapshot in run_default_collision_check after a db.rs regeneration"]
    fn dump_default_collision_report_as_builder_code() {
        let join_handle = std::thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .spawn(dump_default_collision_report)
            .unwrap();
        join_handle.join().unwrap();
    }

    fn dump_default_collision_report() {
        use crate::identity::hotkey_token::HotkeyToken;
        use crate::unit::grids::PositionCollisionCard;
        let default_text = crate::DEFAULT_CUSTOM_KEYS;
        let custom_keys = CustomKeys::from_text(default_text);
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        for entry in report.entries() {
            let unit_id = entry.unit_id().value();
            let unit_name = entry.unit_name();
            let position_cards = entry.position_cards();
            let hotkey_cards = entry.hotkey_cards();
            let main_position = position_cards[0];
            let secondary_position = position_cards[1];
            let main_hotkey = hotkey_cards[0];
            let secondary_hotkey = hotkey_cards[1];
            println!("// {unit_id} ({unit_name})");
            println!("let entry = {{");
            if !main_position.is_empty() {
                emit_position_card_builder("main_pos", &main_position);
            }
            if !secondary_position.is_empty() {
                emit_position_card_builder("secondary_pos", &secondary_position);
            }
            if !main_hotkey.is_empty() {
                emit_hotkey_card_builder("main_hot", &main_hotkey);
            }
            if !secondary_hotkey.is_empty() {
                emit_hotkey_card_builder("secondary_hot", &secondary_hotkey);
            }
            println!(
                "    let eb = UnitCollisionEntryBuilder::new(\"{unit_id}\", \"{unit_name}\", empty_pos, empty_hot);",
            );
            if !main_position.is_empty() {
                println!("    let eb = eb.main_position_card(main_pos);");
            }
            if !main_hotkey.is_empty() {
                println!("    let eb = eb.main_hotkey_card(main_hot);");
            }
            if !secondary_hotkey.is_empty() {
                println!("    let eb = eb.secondary_hotkey_card(secondary_hot);");
            }
            println!("    eb.build()");
            println!("}};");
            println!("builder = builder.entry(entry);");
            println!();
        }

        fn role_expr(role: crate::unit::grids::GridRole) -> &'static str {
            match role {
                crate::unit::grids::GridRole::MainCommand => "GridRole::MainCommand",
                crate::unit::grids::GridRole::HeroSkillTree => "GridRole::HeroSkillTree",
                crate::unit::grids::GridRole::BuildMenu => "GridRole::BuildMenu",
                crate::unit::grids::GridRole::UprootedForm => "GridRole::UprootedForm",
            }
        }

        fn emit_position_card_builder(name: &str, card: &PositionCollisionCard) {
            let role_text = role_expr(card.role());
            println!("    let {name} = PositionCollisionCardBuilder::new({role_text})");
            for (position, slots) in card.into_iter() {
                let column_u8 = u8::from(position.column());
                let row_u8 = u8::from(position.row());
                let slot_idents: Vec<String> = slots
                    .iter()
                    .map(|slot| format!("GridSlotId::ability(\"{}\")", slot.as_str()))
                    .collect();
                let slots_array = slot_idents.join(", ");
                println!("        .collision_at({column_u8}, {row_u8}, &[{slots_array}])",);
            }
            println!("        .build();");
        }

        fn emit_hotkey_card_builder(name: &str, card: &crate::unit::grids::HotkeyCollisionCard) {
            let role_text = role_expr(card.role());
            println!("    let {name} = HotkeyCollisionCardBuilder::new({role_text}, layout)",);
            for (_position, cell) in card.into_iter() {
                let token = cell.token();
                let letter_char = match token {
                    HotkeyToken::Letter(letter) => letter.character(),
                    _ => continue,
                };
                let slot_idents: Vec<String> = cell
                    .slots()
                    .iter()
                    .map(|slot| format!("GridSlotId::ability(\"{}\")", slot.as_str()))
                    .collect();
                let slots_array = slot_idents.join(", ");
                println!("        .collision('{letter_char}', &[{slots_array}])");
            }
            println!("        .build();");
        }
    }

    fn run_default_collision_check() {
        let default_text = crate::DEFAULT_CUSTOM_KEYS;
        let custom_keys = CustomKeys::from_text(default_text);
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let empty_pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
        let empty_pos = empty_pos_builder.build();
        let empty_hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
        let empty_hot = empty_hot_builder.build();
        let mut builder = UnitCollisionReportBuilder::new();
        // nahy (Ancient Hydra)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    1,
                    2,
                    &[
                        crate::test_support::ability_slot("Awrh"),
                        crate::test_support::ability_slot("Aspo"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'X',
                    &[
                        crate::test_support::ability_slot("Awrh"),
                        crate::test_support::ability_slot("Aspo"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nahy", "Ancient Hydra", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsqa (Ancient Sasquatch)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    1,
                    2,
                    &[
                        crate::test_support::ability_slot("ACfr"),
                        crate::test_support::ability_slot("ACtc"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'X',
                    &[
                        crate::test_support::ability_slot("ACfr"),
                        crate::test_support::ability_slot("ACtc"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nsqa", "Ancient Sasquatch", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // eden (Ancient of Wonders)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    3,
                    2,
                    &[
                        crate::test_support::ability_slot("Anei"),
                        crate::test_support::ability_slot("Aro1"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("eden", "Ancient of Wonders", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // espv (Avatar of Vengeance)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'V',
                    &[
                        crate::test_support::ability_slot("Avng"),
                        crate::test_support::ability_slot("ACrk"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("espv", "Avatar of Vengeance", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // obar (Barracks)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[
                        crate::test_support::ability_slot("ogru"),
                        crate::test_support::ability_slot("ocat"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("obar", "Barracks", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Orex (Beastmaster)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'S',
                    &[
                        crate::test_support::ability_slot("CmdStop"),
                        crate::test_support::ability_slot("Aamk"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Orex", "Beastmaster", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // unp2 (Black Citadel)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    3,
                    0,
                    &[
                        crate::test_support::ability_slot("Rupm"),
                        crate::test_support::ability_slot("CmdAttack"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("unp2", "Black Citadel", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nadr (Blue Dragon)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("Afrc"),
                        crate::test_support::ability_slot("ACdv"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nadr", "Blue Dragon", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nbdo (Blue Dragonspawn Overseer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    2,
                    &[
                        crate::test_support::ability_slot("ACav"),
                        crate::test_support::ability_slot("ACev"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("ACav"),
                        crate::test_support::ability_slot("ACev"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "nbdo",
                "Blue Dragonspawn Overseer",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Nsjs (Brewmaster)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'S',
                    &[
                        crate::test_support::ability_slot("CmdStop"),
                        crate::test_support::ability_slot("Aamk"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Nsjs", "Brewmaster", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsbm (Brood Mother)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACen"),
                        crate::test_support::ability_slot("ACvs"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[
                        crate::test_support::ability_slot("ACen"),
                        crate::test_support::ability_slot("ACvs"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nsbm", "Brood Mother", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ncks (Centaur Sorcerer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACdm"),
                        crate::test_support::ability_slot("ACbl"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("ncks", "Centaur Sorcerer", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nchp (Chaplain)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("Adsm"),
                        crate::test_support::ability_slot("Anh2"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nchp", "Chaplain", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // ndth (Dark Troll High Priest)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACdm"),
                        crate::test_support::ability_slot("Anh2"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "ndth",
                "Dark Troll High Priest",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nwzd (Dark Wizard)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    2,
                    &[
                        crate::test_support::ability_slot("ACpy"),
                        crate::test_support::ability_slot("ACba"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("ACpy"),
                        crate::test_support::ability_slot("ACba"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nwzd", "Dark Wizard", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Udea (Death Knight)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("AUdc"),
                        crate::test_support::ability_slot("AUau"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Udea", "Death Knight", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Uear (Death Knight)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("AUdc"),
                        crate::test_support::ability_slot("AUau"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Uear", "Death Knight", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nrvd (Death Revenant)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACdc"),
                        crate::test_support::ability_slot("ACrd"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nrvd", "Death Revenant", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Ecen (Demigod)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'S',
                    &[
                        crate::test_support::ability_slot("CmdStop"),
                        crate::test_support::ability_slot("SCc1"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Ecen", "Demigod", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Eevi (Demon Hunter)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    0,
                    &[
                        crate::test_support::ability_slot("CmdHoldPos"),
                        crate::test_support::ability_slot("ANcl"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Eevi", "Demon Hunter", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Eevm (Demon Hunter)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    0,
                    &[
                        crate::test_support::ability_slot("CmdHoldPos"),
                        crate::test_support::ability_slot("ANcl"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Eevm", "Demon Hunter", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // ubsp (Destroyer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("Aabs"),
                        crate::test_support::ability_slot("Advm"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ubsp", "Destroyer", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nba2 (Doom Guard)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'F',
                    &[
                        crate::test_support::ability_slot("ACsk"),
                        crate::test_support::ability_slot("ACrf"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nba2", "Doom Guard", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nbal (Doom Guard)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'F',
                    &[
                        crate::test_support::ability_slot("ACsk"),
                        crate::test_support::ability_slot("ACrf"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nbal", "Doom Guard", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ndh3 (Draenei Barracks)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    0,
                    &[
                        crate::test_support::ability_slot("ndrt"),
                        crate::test_support::ability_slot("ndrn"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'E',
                    &[
                        crate::test_support::ability_slot("ndrt"),
                        crate::test_support::ability_slot("ndrn"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("ndh3", "Draenei Barracks", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ndrs (Draenei Seer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("AChv"),
                        crate::test_support::ability_slot("ACsw"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ndrs", "Draenei Seer", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Ubal (Dreadlord)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'E',
                    &[
                        crate::test_support::ability_slot("AUsl"),
                        crate::test_support::ability_slot("AOeq"),
                    ],
                )
                .build();
            let secondary_hot = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout)
                .collision(
                    'E',
                    &[
                        crate::test_support::ability_slot("AUsl"),
                        crate::test_support::ability_slot("AOeq"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Ubal", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            let eb = eb.secondary_hotkey_card(secondary_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Udre (Dreadlord)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("AUcs"),
                        crate::test_support::ability_slot("AUav"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Udre", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Udth (Dreadlord)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'D',
                    &[
                        crate::test_support::ability_slot("AEsh"),
                        crate::test_support::ability_slot("AUdd"),
                    ],
                )
                .build();
            let secondary_hot = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout)
                .collision(
                    'D',
                    &[
                        crate::test_support::ability_slot("AEsh"),
                        crate::test_support::ability_slot("AUdd"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Udth", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            let eb = eb.secondary_hotkey_card(secondary_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Umal (Dreadlord)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("AUcs"),
                        crate::test_support::ability_slot("ANdc"),
                    ],
                )
                .build();
            let secondary_hot = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout)
                .collision(
                    'E',
                    &[
                        crate::test_support::ability_slot("AUsl"),
                        crate::test_support::ability_slot("ANdc"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Umal", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            let eb = eb.secondary_hotkey_card(secondary_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Utic (Dreadlord)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    1,
                    2,
                    &[
                        crate::test_support::ability_slot("ANrc"),
                        crate::test_support::ability_slot("AUsl"),
                    ],
                )
                .collision_at(
                    3,
                    2,
                    &[
                        crate::test_support::ability_slot("AUin"),
                        crate::test_support::ability_slot("ANfd"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Utic", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Uvng (Dreadlord)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("AUcs"),
                        crate::test_support::ability_slot("AUav"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Uvng", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // npn3 (Earth)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ANta"),
                        crate::test_support::ability_slot("ACpv"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("npn3", "Earth", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // npn6 (Earth)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ANta"),
                        crate::test_support::ability_slot("ACpv"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("npn6", "Earth", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nvde (Elder Voidwalker)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACde"),
                        crate::test_support::ability_slot("ACfl"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[
                        crate::test_support::ability_slot("ACde"),
                        crate::test_support::ability_slot("ACfl"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nvde", "Elder Voidwalker", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nerd (Eredar Diabolist)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ANfb"),
                        crate::test_support::ability_slot("ACpa"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[
                        crate::test_support::ability_slot("ANfb"),
                        crate::test_support::ability_slot("ACpa"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nerd", "Eredar Diabolist", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nfot (Faceless One Terror)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    1,
                    2,
                    &[
                        crate::test_support::ability_slot("ACmf"),
                        crate::test_support::ability_slot("ACsl"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'X',
                    &[
                        crate::test_support::ability_slot("ACmf"),
                        crate::test_support::ability_slot("ACsl"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nfot", "Faceless One Terror", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nfor (Faceless One Trickster)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACpu"),
                        crate::test_support::ability_slot("ACcs"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "nfor",
                "Faceless One Trickster",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // npfm (Fel Ravager)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACde"),
                        crate::test_support::ability_slot("ACbk"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[
                        crate::test_support::ability_slot("ACde"),
                        crate::test_support::ability_slot("ACbk"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("npfm", "Fel Ravager", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nfsp (Forest Troll Shadow Priest)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACdm"),
                        crate::test_support::ability_slot("Anh1"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "nfsp",
                "Forest Troll Shadow Priest",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nfgo (Forgotten One)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACtn"),
                        crate::test_support::ability_slot("ACfb"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[
                        crate::test_support::ability_slot("ACtn"),
                        crate::test_support::ability_slot("ACfb"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nfgo", "Forgotten One", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ugar (Gargoyle)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[
                        crate::test_support::ability_slot("CmdMove"),
                        crate::test_support::ability_slot("Aatp"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ugar", "Gargoyle", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsgt (Giant Spider)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACen"),
                        crate::test_support::ability_slot("ACvs"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[
                        crate::test_support::ability_slot("ACen"),
                        crate::test_support::ability_slot("ACvs"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nsgt", "Giant Spider", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ngow (Gnoll Warlord)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACro"),
                        crate::test_support::ability_slot("ACbl"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[
                        crate::test_support::ability_slot("ACro"),
                        crate::test_support::ability_slot("ACbl"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ngow", "Gnoll Warlord", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ngad (Goblin Laboratory)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[
                        crate::test_support::ability_slot("ngsp"),
                        crate::test_support::ability_slot("nzep"),
                        crate::test_support::ability_slot("Andt"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("ngad", "Goblin Laboratory", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // ngme (Goblin Merchant)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'E',
                    &[
                        crate::test_support::ability_slot("bspd"),
                        crate::test_support::ability_slot("stel"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("ngme", "Goblin Merchant", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // unp1 (Halls of the Dead)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    3,
                    0,
                    &[
                        crate::test_support::ability_slot("Rupm"),
                        crate::test_support::ability_slot("CmdAttack"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("unp1", "Halls of the Dead", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nhhr (Heretic)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACca"),
                        crate::test_support::ability_slot("ACrd"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nhhr", "Heretic", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nheb (High Elven Barracks)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'D',
                    &[
                        crate::test_support::ability_slot("nws1"),
                        crate::test_support::ability_slot("Rhde"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nheb", "High Elven Barracks", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nith (Ice Troll High Priest)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    1,
                    2,
                    &[
                        crate::test_support::ability_slot("ACd2"),
                        crate::test_support::ability_slot("ACf2"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "nith",
                "Ice Troll High Priest",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nkog (Kobold Geomancer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACdm"),
                        crate::test_support::ability_slot("ACsw"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nkog", "Kobold Geomancer", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nrvl (Lightning Revenant)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACcl"),
                        crate::test_support::ability_slot("ACpu"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nrvl", "Lightning Revenant", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // umtw (Meat Wagon)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("Amel"),
                        crate::test_support::ability_slot("Apts"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("umtw", "Meat Wagon", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nmr4 (Mercenary Camp)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[
                        crate::test_support::ability_slot("ncea"),
                        crate::test_support::ability_slot("ncen"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'W',
                    &[
                        crate::test_support::ability_slot("nhrw"),
                        crate::test_support::ability_slot("nqbh"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nmr4", "Mercenary Camp", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nmrd (Mercenary Camp)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[
                        crate::test_support::ability_slot("ntkh"),
                        crate::test_support::ability_slot("nbdw"),
                        crate::test_support::ability_slot("nubw"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nmrd", "Mercenary Camp", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Hmbr (Mountain King)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("AHtc"),
                        crate::test_support::ability_slot("AHbh"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Hmbr", "Mountain King", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Hmkg (Mountain King)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("AHtc"),
                        crate::test_support::ability_slot("AHbh"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Hmkg", "Mountain King", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nmsn (Mur'gul Snarecaster)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACdm"),
                        crate::test_support::ability_slot("ACsw"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nmsn", "Mur'gul Snarecaster", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nnrg (Naga Royal Guard)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACcb"),
                        crate::test_support::ability_slot("ACcv"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[
                        crate::test_support::ability_slot("ACcb"),
                        crate::test_support::ability_slot("ACcv"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nnrg", "Naga Royal Guard", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nnwq (Nerubian Queen)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACrd"),
                        crate::test_support::ability_slot("ACca"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nnwq", "Nerubian Queen", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nnwr (Nerubian Seer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACdm"),
                        crate::test_support::ability_slot("ACrd"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nnwr", "Nerubian Seer", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nnwl (Nerubian Webspinner)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACrd"),
                        crate::test_support::ability_slot("ACwb"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nnwl", "Nerubian Webspinner", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nndr (Nether Dragon)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    2,
                    &[
                        crate::test_support::ability_slot("ACcr"),
                        crate::test_support::ability_slot("ACmi"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("ACcr"),
                        crate::test_support::ability_slot("ACmi"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nndr", "Nether Dragon", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Nman (Pit Lord)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("AHtc"),
                        crate::test_support::ability_slot("ANrn"),
                    ],
                )
                .build();
            let secondary_hot = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout)
                .collision(
                    'E',
                    &[
                        crate::test_support::ability_slot("ANrn"),
                        crate::test_support::ability_slot("AOeq"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Nman", "Pit Lord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            let eb = eb.secondary_hotkey_card(secondary_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Npld (Pit Lord)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("AHtc"),
                        crate::test_support::ability_slot("ANrn"),
                    ],
                )
                .build();
            let secondary_hot = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout)
                .collision(
                    'E',
                    &[
                        crate::test_support::ability_slot("ANrn"),
                        crate::test_support::ability_slot("AOeq"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Npld", "Pit Lord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            let eb = eb.secondary_hotkey_card(secondary_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nenp (Poison Treant)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACvs"),
                        crate::test_support::ability_slot("Aenr"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[
                        crate::test_support::ability_slot("ACvs"),
                        crate::test_support::ability_slot("Aenr"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nenp", "Poison Treant", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nfpe (Polar Furbolg Elder Shaman)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("AChv"),
                        crate::test_support::ability_slot("ACfn"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "nfpe",
                "Polar Furbolg Elder Shaman",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Emoo (Priestess of the Moon)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("AEst"),
                        crate::test_support::ability_slot("AEar"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "Emoo",
                "Priestess of the Moon",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Etyr (Priestess of the Moon)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("AEst"),
                        crate::test_support::ability_slot("AEar"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "Etyr",
                "Priestess of the Moon",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Hvwd (Ranger)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("AEst"),
                        crate::test_support::ability_slot("AEar"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Hvwd", "Ranger", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nslv (Salamander Vizier)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("Ambd"),
                        crate::test_support::ability_slot("ACdm"),
                        crate::test_support::ability_slot("ACbl"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[
                        crate::test_support::ability_slot("Ambd"),
                        crate::test_support::ability_slot("ACbl"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nslv", "Salamander Vizier", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nstl (Satyr Soulstealer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACrd"),
                        crate::test_support::ability_slot("Ambd"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nstl", "Satyr Soulstealer", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsgb (Sea Giant Behemoth)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACtb"),
                        crate::test_support::ability_slot("ACpv"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nsgb", "Sea Giant Behemoth", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsgh (Sea Giant Hunter)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACen"),
                        crate::test_support::ability_slot("ACpv"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[
                        crate::test_support::ability_slot("ACen"),
                        crate::test_support::ability_slot("ACpv"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nsgh", "Sea Giant Hunter", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ndh4 (Seer's Den)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[
                        crate::test_support::ability_slot("ndrs"),
                        crate::test_support::ability_slot("ndrh"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Q',
                    &[
                        crate::test_support::ability_slot("ndrs"),
                        crate::test_support::ability_slot("ndrh"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ndh4", "Seer's Den", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Orkn (Shadow Hunter)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'S',
                    &[
                        crate::test_support::ability_slot("CmdStop"),
                        crate::test_support::ability_slot("Aamk"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Orkn", "Shadow Hunter", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nnsa (Shrine of Azshara)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[
                        crate::test_support::ability_slot("nnsw"),
                        crate::test_support::ability_slot("nwgs"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nnsa", "Shrine of Azshara", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsoc (Skeletal Orc Champion)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    2,
                    &[
                        crate::test_support::ability_slot("ACvp"),
                        crate::test_support::ability_slot("ACcr"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("ACvp"),
                        crate::test_support::ability_slot("ACcr"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "nsoc",
                "Skeletal Orc Champion",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ngos (Snarlmane the Bloodgorger)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACac"),
                        crate::test_support::ability_slot("ACbl"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[
                        crate::test_support::ability_slot("ACac"),
                        crate::test_support::ability_slot("ACbl"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "ngos",
                "Snarlmane the Bloodgorger",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nnsg (Spawning Grounds)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[
                        crate::test_support::ability_slot("nmyr"),
                        crate::test_support::ability_slot("nsnp"),
                        crate::test_support::ability_slot("nhyc"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nnsg", "Spawning Grounds", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // hspt (Spellbreaker)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("Acmg"),
                        crate::test_support::ability_slot("Amim"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("hspt", "Spellbreaker", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ospm (Spirit Walker)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'F',
                    &[
                        crate::test_support::ability_slot("ACsk"),
                        crate::test_support::ability_slot("Acpf"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ospm", "Spirit Walker", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // noga (Stonemaul Warchief)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    2,
                    &[
                        crate::test_support::ability_slot("ACbh"),
                        crate::test_support::ability_slot("SCae"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("ACbh"),
                        crate::test_support::ability_slot("SCae"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("noga", "Stonemaul Warchief", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // npn2 (Storm)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ANwk"),
                        crate::test_support::ability_slot("Adsm"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("npn2", "Storm", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // npn5 (Storm)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ANwk"),
                        crate::test_support::ability_slot("Adsm"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("npn5", "Storm", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nstw (Storm Wyrm)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACdv"),
                        crate::test_support::ability_slot("ACcl"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nstw", "Storm Wyrm", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsrn (Stormreaver Necrolyte)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACcl"),
                        crate::test_support::ability_slot("ACbl"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[
                        crate::test_support::ability_slot("ACcl"),
                        crate::test_support::ability_slot("ACbl"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "nsrn",
                "Stormreaver Necrolyte",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Ocb2 (Tauren Chieftain)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'S',
                    &[
                        crate::test_support::ability_slot("CmdStop"),
                        crate::test_support::ability_slot("Aamk"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("Ocb2", "Tauren Chieftain", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nntt (Temple of Tides)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[
                        crate::test_support::ability_slot("nmpe"),
                        crate::test_support::ability_slot("nnmg"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nntt", "Temple of Tides", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Nrob (Tinker)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'S',
                    &[
                        crate::test_support::ability_slot("CmdStop"),
                        crate::test_support::ability_slot("ANde"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Nrob", "Tinker", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ntkh (Tuskarr Healer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("Anh1"),
                        crate::test_support::ability_slot("ACdm"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ntkh", "Tuskarr Healer", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Ewar (Warden)
        let entry = {
            let secondary_hot = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout)
                .collision(
                    'D',
                    &[
                        crate::test_support::ability_slot("AEsh"),
                        crate::test_support::ability_slot("AIhm"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Ewar", "Warden", empty_pos, empty_hot);
            let eb = eb.secondary_hotkey_card(secondary_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Ewrd (Warden)
        let entry = {
            let secondary_hot = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout)
                .collision(
                    'D',
                    &[
                        crate::test_support::ability_slot("AEsh"),
                        crate::test_support::ability_slot("AIhm"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Ewrd", "Warden", empty_pos, empty_hot);
            let eb = eb.secondary_hotkey_card(secondary_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Uwar (Warlock)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'S',
                    &[
                        crate::test_support::ability_slot("CmdStop"),
                        crate::test_support::ability_slot("ACm2"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Uwar", "Warlock", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsns (Watery Minion)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        crate::test_support::ability_slot("ACdm"),
                        crate::test_support::ability_slot("ACsw"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nsns", "Watery Minion", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // ngh2 (Wraith)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[
                        crate::test_support::ability_slot("ACcs"),
                        crate::test_support::ability_slot("ACps"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ngh2", "Wraith", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);
        let expected = builder.build();
        assert_eq!(
            report, expected,
            "default CustomKeys.txt collision report changed — update the expected entries if intentional",
        );
    }
}
