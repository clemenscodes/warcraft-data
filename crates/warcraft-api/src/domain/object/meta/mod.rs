//! [`WarcraftObjectMeta`]: the kind-specific metadata carried by a
//! `WarcraftObject`.

use crate::domain::ability::AbilityMeta;
use crate::domain::command::CommandMeta;
use crate::domain::item::ItemMeta;
use crate::domain::unit::UnitMeta;
use crate::domain::upgrade::UpgradeMeta;

#[derive(Debug, Clone)]
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

// NOTE: `WarcraftObjectMeta` becomes a `ValueObject` once its float-bearing
// variants (UnitMeta/AbilityMeta) are Eq-able via the fixed-point quantity VOs
// (slice 3). Until then it carries no role marker.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warcraft_object_meta_default_is_unit_variant() {
        matches!(WarcraftObjectMeta::default(), WarcraftObjectMeta::Unit(_));
    }
}
