//! Object identity. This is the DDD `Identifier` of the object aggregate — a
//! *domain* model of identity, deliberately distinct from the game's in-memory
//! `Identifier` primitive (`warcraft-primitives`): here an id is a static,
//! already-known string that only the crate's own authoritative data may mint.

use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

/// A Warcraft III object identity (unit, ability, upgrade, item, command, or
/// system-keybind section id).
///
/// Identity is **case-insensitive**: the auto-generated database registers some
/// objects under mixed casing (a unit may list `Acvs` while the object is stored
/// `ACvs`), so `PartialEq`/`Eq`/`Hash`/`Ord` all fold ASCII case. Plain `==` and
/// id-keyed `HashMap`/`BTreeMap`/`HashSet` therefore merge casing variants
/// automatically — no `.eq_ignore_ascii_case`/`.to_ascii_lowercase` folding is
/// needed anywhere else. Because equality ignores case, this type deliberately
/// does **not** implement `Borrow<str>` (that would expose the case-sensitive
/// `str` impls and make map lookups unsound).
#[derive(Default, Debug, Copy, Clone)]
pub struct WarcraftObjectId {
    pub(crate) value: &'static str,
}

impl WarcraftObjectId {
    /// Mint an id from a static string. This is **`pub(crate)`**: only this
    /// crate's own authoritative database data (the generated `db.rs` and the
    /// `const`/`static` id tables) may call it. No downstream crate can
    /// construct a `WarcraftObjectId` from an arbitrary string; they must obtain
    /// ids from the API instead. This makes it impossible to fabricate an id
    /// whose value is not a real object id.
    pub(crate) const fn new(value: &'static str) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &'static str {
        self.value
    }
}

impl PartialEq for WarcraftObjectId {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq_ignore_ascii_case(other.value)
    }
}

impl Eq for WarcraftObjectId {}

impl Hash for WarcraftObjectId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for byte in self.value.bytes() {
            let folded = byte.to_ascii_lowercase();
            state.write_u8(folded);
        }
        state.write_u8(0xff);
    }
}

impl PartialOrd for WarcraftObjectId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WarcraftObjectId {
    fn cmp(&self, other: &Self) -> Ordering {
        let own = self.value.bytes().map(|byte| byte.to_ascii_lowercase());
        let their = other.value.bytes().map(|byte| byte.to_ascii_lowercase());
        own.cmp(their)
    }
}

impl fmt::Display for WarcraftObjectId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.value)
    }
}

// DDD role: the identity of the object aggregate.
impl ddd::Layered for WarcraftObjectId {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for WarcraftObjectId {}
impl ddd::Identifier for WarcraftObjectId {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warcraft_object_id_value_round_trips() {
        let id = WarcraftObjectId::new("hpea");
        assert_eq!(id.value(), "hpea");
    }

    #[test]
    fn warcraft_object_id_ordering_is_lexicographic() {
        let alpha = WarcraftObjectId::new("Aaaa");
        let beta = WarcraftObjectId::new("Zzzz");
        assert!(alpha < beta);
    }

    #[test]
    fn warcraft_object_id_equality_ignores_ascii_case() {
        let mixed = WarcraftObjectId::new("ACvs");
        let lower = WarcraftObjectId::new("acvs");
        assert_eq!(mixed, lower);
    }

    #[test]
    fn warcraft_object_id_hash_ignores_ascii_case() {
        use std::collections::HashSet;
        let mut ids = HashSet::new();
        ids.insert(WarcraftObjectId::new("ACvs"));
        assert!(ids.contains(&WarcraftObjectId::new("acvs")));
    }

    #[test]
    fn warcraft_object_id_ordering_ignores_ascii_case() {
        let alpha = WarcraftObjectId::new("acad");
        let beta = WarcraftObjectId::new("ACAE");
        assert!(alpha < beta);
    }
}
