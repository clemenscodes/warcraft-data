//! Pure chain building: turns the data tables plus the extracted [`UnitFacts`]
//! into the raw evidence chains (each ordered weakest → strongest) that the
//! build merges into groups. No database, no globals — every function takes only
//! the minimal id sets it needs, so each rule is exercised in isolation.

use std::collections::{BTreeMap, HashSet};

use crate::application::unit::variant::facts::UnitFacts;
use crate::domain::identity::WarcraftObjectId;

/// Gather every evidence chain: tier groups and the curated list (both already
/// merged into `tier_groups`), upgrade-swaps, and hero name grouping. Every
/// returned chain has at least two members.
pub(crate) fn chains(
    tier_groups: &[&[WarcraftObjectId]],
    upgrade_swaps: &[(WarcraftObjectId, WarcraftObjectId)],
    facts: &UnitFacts,
) -> Vec<Vec<WarcraftObjectId>> {
    let tiers = tier_groups
        .iter()
        .filter_map(|group| tier_chain(group, &facts.non_hero_units));
    let swaps = upgrade_swaps
        .iter()
        .filter_map(|(from_id, to_id)| swap_chain(*from_id, *to_id, &facts.non_hero_units));
    let heroes = hero_chains(&facts.heroes_by_name, &facts.produced_units);
    tiers.chain(swaps).chain(heroes).collect()
}

/// A tier group filtered to real non-hero units, preserving order. `None` when
/// fewer than two members survive (no relation to record).
fn tier_chain(
    group: &[WarcraftObjectId],
    non_hero_units: &HashSet<WarcraftObjectId>,
) -> Option<Vec<WarcraftObjectId>> {
    let members: Vec<WarcraftObjectId> = group
        .iter()
        .copied()
        .filter(|id| non_hero_units.contains(id))
        .collect();
    (members.len() >= 2).then_some(members)
}

/// An upgrade-swap as a `[from, to]` chain, kept only when both ends are real
/// non-hero units.
fn swap_chain(
    from_id: WarcraftObjectId,
    to_id: WarcraftObjectId,
    non_hero_units: &HashSet<WarcraftObjectId>,
) -> Option<Vec<WarcraftObjectId>> {
    let both_real = non_hero_units.contains(&from_id) && non_hero_units.contains(&to_id);
    both_real.then(|| vec![from_id, to_id])
}

/// Every hero name group turned into a chain.
fn hero_chains(
    heroes_by_name: &BTreeMap<&'static str, Vec<WarcraftObjectId>>,
    produced_units: &HashSet<WarcraftObjectId>,
) -> Vec<Vec<WarcraftObjectId>> {
    heroes_by_name
        .values()
        .filter_map(|hero_ids| hero_name_chain(produced_units, hero_ids.clone()))
        .collect()
}

/// One name group → its chain, or `None` when the canonical is ambiguous. Needs
/// at least two ids and **exactly one** produced member (zero = campaign-only,
/// more than one = ambiguous — both left ungrouped). The chain is the other ids
/// sorted ascending, with the produced canonical pushed last.
fn hero_name_chain(
    produced_units: &HashSet<WarcraftObjectId>,
    hero_ids: Vec<WarcraftObjectId>,
) -> Option<Vec<WarcraftObjectId>> {
    if hero_ids.len() < 2 {
        return None;
    }
    let produced_members: Vec<WarcraftObjectId> = hero_ids
        .iter()
        .copied()
        .filter(|id| produced_units.contains(id))
        .collect();
    let [canonical] = produced_members[..] else {
        return None;
    };
    let mut chain: Vec<WarcraftObjectId> =
        hero_ids.into_iter().filter(|id| *id != canonical).collect();
    chain.sort_unstable();
    chain.push(canonical);
    Some(chain)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &'static str) -> WarcraftObjectId {
        WarcraftObjectId::new(value)
    }

    #[test]
    fn tier_chain_keeps_mergeable_members_in_order() {
        let non_hero = HashSet::from([id("aa1"), id("aa2"), id("aa3")]);
        let chain = tier_chain(&[id("aa1"), id("aa2"), id("aa3")], &non_hero);
        assert_eq!(chain, Some(vec![id("aa1"), id("aa2"), id("aa3")]));
    }

    #[test]
    fn tier_chain_drops_non_mergeable_members() {
        // aa2 is not a non-hero unit → filtered; two survive.
        let non_hero = HashSet::from([id("aa1"), id("aa3")]);
        let chain = tier_chain(&[id("aa1"), id("aa2"), id("aa3")], &non_hero);
        assert_eq!(chain, Some(vec![id("aa1"), id("aa3")]));
    }

    #[test]
    fn tier_chain_is_none_when_fewer_than_two_survive() {
        let non_hero = HashSet::from([id("aa1")]);
        assert_eq!(tier_chain(&[id("aa1"), id("aa2")], &non_hero), None);
    }

    #[test]
    fn swap_chain_is_from_then_to_when_both_mergeable() {
        let non_hero = HashSet::from([id("ohun"), id("otbk")]);
        assert_eq!(
            swap_chain(id("ohun"), id("otbk"), &non_hero),
            Some(vec![id("ohun"), id("otbk")])
        );
    }

    #[test]
    fn swap_chain_is_none_when_either_end_is_not_mergeable() {
        let non_hero = HashSet::from([id("ohun")]);
        assert_eq!(swap_chain(id("ohun"), id("Nal2"), &non_hero), None);
    }

    #[test]
    fn hero_name_chain_puts_the_single_produced_member_last() {
        let produced = HashSet::from([id("Nalc")]);
        let chain = hero_name_chain(&produced, vec![id("Nalm"), id("Nalc"), id("Nal2")]);
        assert_eq!(chain, Some(vec![id("Nal2"), id("Nalm"), id("Nalc")]));
    }

    #[test]
    fn hero_name_chain_is_none_without_exactly_one_produced_member() {
        assert_eq!(
            hero_name_chain(&HashSet::new(), vec![id("Ha1"), id("Ha2")]),
            None,
            "zero produced is ambiguous",
        );
        let two_produced = HashSet::from([id("Ha1"), id("Ha2")]);
        assert_eq!(
            hero_name_chain(&two_produced, vec![id("Ha1"), id("Ha2")]),
            None,
            "two produced is ambiguous",
        );
    }

    #[test]
    fn hero_name_chain_is_none_for_a_lone_id() {
        let produced = HashSet::from([id("Ha1")]);
        assert_eq!(hero_name_chain(&produced, vec![id("Ha1")]), None);
    }

    #[test]
    fn chains_gathers_tiers_swaps_and_hero_groups() {
        let facts = UnitFacts {
            non_hero_units: HashSet::from([id("aa1"), id("aa2"), id("ohun"), id("otbk")]),
            heroes_by_name: BTreeMap::from([("Alchemist", vec![id("Nal2"), id("Nalc")])]),
            produced_units: HashSet::from([id("Nalc")]),
        };
        let tier_groups: &[&[WarcraftObjectId]] = &[&[id("aa1"), id("aa2")]];
        let swaps = [(id("ohun"), id("otbk"))];

        let result = chains(tier_groups, &swaps, &facts);

        assert!(result.contains(&vec![id("aa1"), id("aa2")]), "tier chain");
        assert!(result.contains(&vec![id("ohun"), id("otbk")]), "swap chain");
        assert!(result.contains(&vec![id("Nal2"), id("Nalc")]), "hero chain");
    }
}
