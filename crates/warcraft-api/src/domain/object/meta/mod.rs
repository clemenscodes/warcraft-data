//! [`WarcraftObjectMeta`]: the kind-specific metadata carried by a
//! `WarcraftObject`.

use crate::domain::ability::AbilityMeta;
use crate::domain::command::CommandMeta;
use crate::domain::item::ItemMeta;
use crate::domain::unit::UnitMeta;
use crate::domain::upgrade::UpgradeMeta;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WarcraftObjectMeta {
    Unit(UnitMeta),
    Ability(AbilityMeta),
    Upgrade(UpgradeMeta),
    Item(ItemMeta),
    Command(CommandMeta),
}

impl Default for WarcraftObjectMeta {
    fn default() -> Self {
        Self::Unit(UnitMeta::default())
    }
}

// DDD role: immutable, equality-by-value → Value Object (now that every variant
// is `Eq` via the fixed-point quantity VOs).
impl ddd::Layered for WarcraftObjectMeta {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for WarcraftObjectMeta {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warcraft_object_meta_default_is_unit_variant() {
        matches!(WarcraftObjectMeta::default(), WarcraftObjectMeta::Unit(_));
    }
}
