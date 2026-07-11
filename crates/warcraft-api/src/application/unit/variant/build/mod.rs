//! Pure assembly of evidence chains into ordered variant groups. Given the raw
//! chains (from `sources`), it (1) walks them to build a union-find plus the
//! weakest→strongest precedence edges, (2) ranks every member by longest path,
//! (3) merges overlapping chains into components and orders each. No database
//! access — same chains in, same groups out.

use std::collections::{HashMap, HashSet};

use crate::application::unit::variant::group::VariantGroup;
use crate::application::unit::variant::union_find::UnionFind;
use crate::domain::identity::WarcraftObjectId;

/// A `predecessor precedes successor` link between two adjacent members of one
/// chain — the evidence used to rank members weakest → strongest when chains
/// merge into a shared component.
struct PrecedenceEdge {
    predecessor: WarcraftObjectId,
    successor: WarcraftObjectId,
}

/// The merge graph derived from the chains: every member once in first-seen
/// order, the union-find that fuses overlapping chains, and the precedence
/// edges.
struct MergeGraph {
    nodes: Vec<WarcraftObjectId>,
    union_find: UnionFind<WarcraftObjectId>,
    edges: Vec<PrecedenceEdge>,
}

/// Merge the chains into ordered variant groups (weakest → strongest, canonical
/// last), deterministically ordered by canonical id. Groups with fewer than two
/// members are dropped.
pub(crate) fn groups(chains: &[Vec<WarcraftObjectId>]) -> Vec<VariantGroup> {
    let graph = walk(chains);
    let ranks = rank(&graph);
    assemble(graph, &ranks)
}

/// Walk every chain: record each member once (first-seen order), union adjacent
/// members so overlapping chains fuse, and emit a precedence edge per step.
fn walk(chains: &[Vec<WarcraftObjectId>]) -> MergeGraph {
    let mut nodes: Vec<WarcraftObjectId> = Vec::new();
    let mut seen: HashSet<WarcraftObjectId> = HashSet::new();
    let mut union_find: UnionFind<WarcraftObjectId> = UnionFind::new();
    let mut edges: Vec<PrecedenceEdge> = Vec::new();

    for chain in chains {
        let mut previous: Option<WarcraftObjectId> = None;
        for member in chain.iter().copied() {
            if seen.insert(member) {
                nodes.push(member);
            }
            if let Some(predecessor) = previous
                && predecessor != member
            {
                union_find.union(predecessor, member);
                edges.push(PrecedenceEdge {
                    predecessor,
                    successor: member,
                });
            }
            previous = Some(member);
        }
    }

    MergeGraph {
        nodes,
        union_find,
        edges,
    }
}

/// Longest-path rank of every member: a source (weakest tier) stays 0, each
/// step toward the strongest adds one. Bellman-Ford-style relaxation capped at
/// the node count, so a malformed cyclic chain can never loop forever.
fn rank(graph: &MergeGraph) -> HashMap<WarcraftObjectId, u32> {
    let mut ranks: HashMap<WarcraftObjectId, u32> =
        graph.nodes.iter().map(|node| (*node, 0)).collect();
    for _ in 0..graph.nodes.len() {
        let mut changed = false;
        for edge in &graph.edges {
            let candidate = ranks.get(&edge.predecessor).copied().unwrap_or(0) + 1;
            let successor_rank = ranks.entry(edge.successor).or_insert(0);
            if candidate > *successor_rank {
                *successor_rank = candidate;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    ranks
}

/// Bucket members by their union-find root, order each bucket by (rank, id),
/// keep only the ≥2-member components, and order the groups by canonical id.
fn assemble(graph: MergeGraph, ranks: &HashMap<WarcraftObjectId, u32>) -> Vec<VariantGroup> {
    let MergeGraph {
        nodes,
        mut union_find,
        ..
    } = graph;

    let mut members_by_root: HashMap<WarcraftObjectId, Vec<WarcraftObjectId>> = HashMap::new();
    for node in nodes {
        let root = union_find.find(node);
        members_by_root.entry(root).or_default().push(node);
    }

    let mut groups: Vec<VariantGroup> = members_by_root
        .into_values()
        .filter_map(|mut members| {
            members.sort_by(|left, right| {
                let left_rank = ranks.get(left).copied().unwrap_or(0);
                let right_rank = ranks.get(right).copied().unwrap_or(0);
                left_rank.cmp(&right_rank).then_with(|| left.cmp(right))
            });
            (members.len() >= 2).then(|| VariantGroup::new(members))
        })
        .collect();
    groups.sort_by_key(|group| group.canonical());
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &'static str) -> WarcraftObjectId {
        WarcraftObjectId::new(value)
    }

    #[test]
    fn a_single_chain_becomes_one_ordered_group() {
        let groups = groups(&[vec![id("aa1"), id("aa2"), id("aa3")]]);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].members(), [id("aa1"), id("aa2"), id("aa3")]);
        assert_eq!(groups[0].canonical(), id("aa3"));
    }

    #[test]
    fn overlapping_chains_union_merge_into_one_group() {
        let groups = groups(&[
            vec![id("bb1"), id("bb2"), id("bb3")],
            vec![id("bb1"), id("bb2"), id("bb3"), id("bb4")],
        ]);
        assert_eq!(groups.len(), 1);
        assert_eq!(
            groups[0].members(),
            [id("bb1"), id("bb2"), id("bb3"), id("bb4")]
        );
        assert_eq!(groups[0].canonical(), id("bb4"));
    }

    #[test]
    fn independent_chains_stay_separate_and_order_by_canonical() {
        let groups = groups(&[vec![id("zz1"), id("zz2")], vec![id("aa1"), id("aa2")]]);
        assert_eq!(groups.len(), 2);
        // Ordered by canonical id: aa2 before zz2.
        assert_eq!(groups[0].canonical(), id("aa2"));
        assert_eq!(groups[1].canonical(), id("zz2"));
    }
}
