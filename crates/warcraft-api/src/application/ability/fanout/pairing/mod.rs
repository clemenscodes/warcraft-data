//! Pure role-based pairing: given the button-positioned abilities of one variant
//! group's members (each tagged with its role — mechanic code plus grid cell),
//! decide which *different-id* abilities must receive the same hotkey/position
//! edit. Abilities pair only when they share a role but differ in id; tiers that
//! reuse one ability id already share a single binding and produce nothing. No
//! database, no globals — pure over the descriptors passed in.

use std::collections::HashMap;

use crate::domain::identity::WarcraftObjectId;

/// An ability's role on a command card: its mechanic `code` plus its default
/// cell. Two abilities are the "same button in the same place" only when all
/// three match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct AbilityRoleKey {
    pub(crate) code: WarcraftObjectId,
    pub(crate) column: u8,
    pub(crate) row: u8,
}

/// One button-positioned ability, tagged with its role.
pub(crate) struct AbilityDescriptor {
    pub(crate) ability_id: WarcraftObjectId,
    pub(crate) role: AbilityRoleKey,
}

/// For one variant group, map each ability id to its same-role, different-id
/// siblings. A role carried by fewer than two distinct ids produces no entry.
pub(crate) fn role_siblings(
    descriptors: &[AbilityDescriptor],
) -> HashMap<WarcraftObjectId, Vec<WarcraftObjectId>> {
    let mut ids_by_role: HashMap<AbilityRoleKey, Vec<WarcraftObjectId>> = HashMap::new();
    for descriptor in descriptors {
        let role_ids = ids_by_role.entry(descriptor.role).or_default();
        if !role_ids.contains(&descriptor.ability_id) {
            role_ids.push(descriptor.ability_id);
        }
    }

    let mut siblings: HashMap<WarcraftObjectId, Vec<WarcraftObjectId>> = HashMap::new();
    for role_ids in ids_by_role.into_values() {
        if role_ids.len() < 2 {
            continue;
        }
        for ability_id in role_ids.iter().copied() {
            let others = role_ids
                .iter()
                .copied()
                .filter(|other| *other != ability_id);
            siblings.entry(ability_id).or_default().extend(others);
        }
    }
    siblings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &'static str) -> WarcraftObjectId {
        WarcraftObjectId::new(value)
    }

    fn role(code: &'static str) -> AbilityRoleKey {
        AbilityRoleKey {
            code: id(code),
            column: 0,
            row: 0,
        }
    }

    fn descriptor(ability: &'static str, code: &'static str) -> AbilityDescriptor {
        AbilityDescriptor {
            ability_id: id(ability),
            role: role(code),
        }
    }

    #[test]
    fn different_ids_in_the_same_role_pair_both_ways() {
        let siblings = role_siblings(&[descriptor("Abu2", "Abur"), descriptor("Abu3", "Abur")]);
        assert_eq!(siblings.get(&id("Abu2")), Some(&vec![id("Abu3")]));
        assert_eq!(siblings.get(&id("Abu3")), Some(&vec![id("Abu2")]));
    }

    #[test]
    fn a_shared_id_across_tiers_produces_no_fanout() {
        let siblings = role_siblings(&[descriptor("Asal", "Aroa"), descriptor("Asal", "Aroa")]);
        assert!(siblings.is_empty());
    }

    #[test]
    fn three_different_ids_each_fan_out_to_the_other_two() {
        let siblings = role_siblings(&[
            descriptor("Asd2", "Asds"),
            descriptor("Asd3", "Asds"),
            descriptor("Asdg", "Asds"),
        ]);
        assert!(siblings[&id("Asdg")].contains(&id("Asd2")));
        assert!(siblings[&id("Asdg")].contains(&id("Asd3")));
        assert!(siblings[&id("Asd2")].contains(&id("Asdg")));
    }

    #[test]
    fn abilities_in_different_roles_never_pair() {
        let siblings = role_siblings(&[descriptor("Aaaa", "Acod"), descriptor("Bbbb", "Bcod")]);
        assert!(siblings.is_empty());
    }
}
