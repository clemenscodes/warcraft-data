//! A generic disjoint-set (union-find) over hashable keys. Knows nothing about
//! units — it merges any elements that share a set. Used by the variant build to
//! fuse evidence chains that overlap on a shared member into one component.

use std::collections::HashMap;
use std::hash::Hash;

pub(crate) struct UnionFind<K> {
    parent: HashMap<K, K>,
}

impl<K: Copy + Eq + Hash> UnionFind<K> {
    pub(crate) fn new() -> Self {
        Self {
            parent: HashMap::new(),
        }
    }

    /// The recorded parent of `node`, defaulting to itself (every unseen key is
    /// its own singleton set).
    fn parent(&self, node: K) -> K {
        self.parent.get(&node).copied().unwrap_or(node)
    }

    /// The representative of `node`'s set, applying path compression so repeated
    /// lookups stay near-constant.
    pub(crate) fn find(&mut self, node: K) -> K {
        let mut root = node;
        while self.parent(root) != root {
            root = self.parent(root);
        }
        let mut current = node;
        while current != root {
            let next = self.parent(current);
            self.parent.insert(current, root);
            current = next;
        }
        root
    }

    /// Merge the sets containing `left` and `right`.
    pub(crate) fn union(&mut self, left: K, right: K) {
        let left_root = self.find(left);
        let right_root = self.find(right);
        if left_root != right_root {
            self.parent.insert(left_root, right_root);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_elements_are_in_their_own_sets() {
        let mut union_find: UnionFind<u32> = UnionFind::new();
        assert_ne!(union_find.find(1), union_find.find(2));
    }

    #[test]
    fn union_merges_sets_transitively() {
        let mut union_find: UnionFind<u32> = UnionFind::new();
        union_find.union(1, 2);
        union_find.union(2, 3);
        assert_eq!(union_find.find(1), union_find.find(3));
        assert_ne!(union_find.find(1), union_find.find(4));
    }
}
