//! Whole-database invariant tests. These exercise the crate from the outside
//! through the public `WarcraftApi` only — they cannot reach the backing store
//! or its type, which is exactly the encapsulation guarantee we want to hold.

use warcraft_api::{
    ItemClass, Race, UnitKind, WarcraftApi, WarcraftObjectKind, WarcraftObjectMeta,
};

/// Exact expected object count — a golden pin, deliberately zero-tolerance.
/// The test fails on ANY deviation, up or down, so that every object a game
/// patch or extraction change adds or removes is surfaced IMMEDIATELY as a red
/// test. When the change is intended, bump this number in the same commit
/// (that's the review record); when it isn't, it's a regression you want to see
/// the moment it happens. (Case-insensitive id collisions merge, so this is the
/// map size, not the raw number of extracted rows.)
const EXPECTED_OBJECT_COUNT: usize = 1742;

/// Object IDs that must always exist. If Blizzard removes or renames any of
/// these, the corresponding test fails and a human has to decide whether to
/// update the anchor set or treat it as a regression.
const ANCHOR_OBJECT_IDS: &[&str] = &[
    "hpea", // Human Peasant
    "htow", // Human Town Hall
    "Hamg", // Human hero: Archmage
    "Hpal", // Human hero: Paladin
    "opeo", // Orc Peon
    "ogre", // Orc Great Hall variant
    "Opgh", // Orc hero: Tauren Chieftain / Far Seer variant
    "uaco", // Undead Acolyte
    "unpl", // Undead Necropolis
    "Udea", // Undead hero: Death Knight
    "etol", // Night Elf Tree of Life
    "Emoo", // Night Elf hero: Keeper of the Grove / Moonkin variant
    "AHbh", // Ability: Paladin Holy Light
    "AHav", // Ability: Paladin Avatar
];

#[test]
fn object_count_matches_exactly() {
    let api = WarcraftApi::default();
    assert_eq!(
        api.len(),
        EXPECTED_OBJECT_COUNT,
        "object count changed — a patch/extraction added or removed objects. \
         If intended, update EXPECTED_OBJECT_COUNT in this commit; otherwise it is a regression."
    );
}

#[test]
fn every_object_has_valid_names_and_icons() {
    let api = WarcraftApi::default();
    for (_, object) in api.iter() {
        assert!(!object.names().is_empty(), "object has no names");
        for name in object.names() {
            assert!(!name.trim().is_empty(), "object contains empty name");
        }
        for icon in object.icons() {
            assert!(
                icon.ends_with(".blp"),
                "icon does not end with .blp: {icon}"
            );
        }
    }
}

#[test]
fn object_ids_are_ascii() {
    let api = WarcraftApi::default();
    for (id, _) in api.iter() {
        let value = id.value();
        assert!(
            value.is_ascii(),
            "object id {value:?} contains non-ASCII characters"
        );
    }
}

#[test]
fn non_command_object_ids_are_three_or_four_chars() {
    let api = WarcraftApi::default();
    for (id, object) in api.iter() {
        if object.kind() == WarcraftObjectKind::Command {
            continue;
        }
        let value = id.value();
        let character_count = value.chars().count();
        assert!(
            (3..=4).contains(&character_count),
            "non-command object id {value:?} has {character_count} chars (expected 3 or 4)"
        );
    }
}

#[test]
fn anchor_objects_are_present() {
    let api = WarcraftApi::default();
    for &anchor_id in ANCHOR_OBJECT_IDS {
        assert!(
            api.by_id(anchor_id).is_some(),
            "anchor object {anchor_id} missing — Blizzard removed/renamed it, or the extractor dropped it"
        );
    }
}

#[test]
fn all_abilities_have_valid_meta() {
    let api = WarcraftApi::default();
    for (id, object) in api.iter() {
        let WarcraftObjectMeta::Ability(ability_meta) = object.meta() else {
            continue;
        };
        let max_level = ability_meta.max_level();
        assert!(
            (1..=4).contains(&max_level),
            "ability {id:?} has invalid max_level {max_level}"
        );
        // base cooldown must be materialized for every ability level
        for level in 1..=max_level {
            assert!(
                ability_meta.cooldown_for_level(level).is_some(),
                "ability {id:?} missing cooldown for level {level}"
            );
        }
    }
}

#[test]
fn ultimate_abilities_have_max_level_one() {
    let api = WarcraftApi::default();
    for (id, object) in api.iter() {
        let WarcraftObjectMeta::Ability(ability_meta) = object.meta() else {
            continue;
        };
        if !ability_meta.is_ultimate() {
            continue;
        }
        assert_eq!(
            ability_meta.max_level(),
            1,
            "ultimate ability {id:?} has max_level {} (expected 1)",
            ability_meta.max_level()
        );
    }
}

#[test]
fn non_ultimate_abilities_have_max_level_at_most_four() {
    let api = WarcraftApi::default();
    for (id, object) in api.iter() {
        let WarcraftObjectMeta::Ability(ability_meta) = object.meta() else {
            continue;
        };
        if ability_meta.is_ultimate() {
            continue;
        }
        assert!(
            ability_meta.max_level() <= 4,
            "non-ultimate ability {id:?} has max_level {} (> 4)",
            ability_meta.max_level()
        );
    }
}

#[test]
fn ability_cooldowns_are_bounded_by_ten_minutes() {
    const MAX_COOLDOWN_MS: u32 = 10 * 60 * 1000;
    let api = WarcraftApi::default();
    for (id, object) in api.iter() {
        let WarcraftObjectMeta::Ability(ability_meta) = object.meta() else {
            continue;
        };
        for (level_index, cooldown_ms) in ability_meta.cooldowns().iter().enumerate() {
            assert!(
                *cooldown_ms <= MAX_COOLDOWN_MS,
                "ability {id:?} level-{} cooldown {cooldown_ms} ms exceeds 10-minute ceiling",
                level_index + 1
            );
        }
    }
}

#[test]
fn unit_build_times_are_reasonable() {
    let api = WarcraftApi::default();
    for (id, object) in api.iter() {
        let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
            continue;
        };
        let build_time = unit_meta.build_time();
        assert!(
            build_time > 0 && build_time < 1000,
            "unit {id:?} has suspicious build time {build_time}"
        );
    }
}

#[test]
fn upgrades_have_at_least_one_level() {
    let api = WarcraftApi::default();
    for (id, object) in api.iter() {
        let WarcraftObjectMeta::Upgrade(upgrade_meta) = object.meta() else {
            continue;
        };
        assert!(
            upgrade_meta.max_level() >= 1,
            "upgrade {id:?} has invalid max_level {}",
            upgrade_meta.max_level()
        );
    }
}

#[test]
fn every_playable_race_has_a_worker() {
    assert_race_has_unit_kind(UnitKind::Worker);
}

#[test]
fn every_playable_race_has_a_hero() {
    assert_race_has_unit_kind(UnitKind::Hero);
}

#[test]
fn every_playable_race_has_a_building() {
    assert_race_has_unit_kind(UnitKind::Building);
}

fn assert_race_has_unit_kind(wanted_kind: UnitKind) {
    let api = WarcraftApi::default();
    for playable_race in [Race::Human, Race::Nightelf, Race::Orc, Race::Undead] {
        let has_kind = api.iter().any(|(_, object)| {
            if object.race() != Some(playable_race) {
                return false;
            }
            let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
                return false;
            };
            unit_meta.unit_kind() == wanted_kind
        });
        assert!(
            has_kind,
            "race {playable_race:?} has no {wanted_kind:?} units"
        );
    }
}

#[test]
fn core_item_classes_are_populated() {
    let api = WarcraftApi::default();
    for expected_class in [ItemClass::Permanent, ItemClass::Charged, ItemClass::PowerUp] {
        let has_item_in_class = api.iter().any(|(_, object)| {
            let WarcraftObjectMeta::Item(item_meta) = object.meta() else {
                return false;
            };
            *item_meta.class() == expected_class
        });
        assert!(
            has_item_in_class,
            "item class {expected_class:?} has no entries"
        );
    }
}
