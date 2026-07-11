//! [`ItemMeta`]: an item's class, granted abilities, and shared cooldown group.

use crate::domain::identity::WarcraftObjectId;
use crate::domain::item::class::ItemClass;

/// Item metadata: the item's class, its granted abilities, and the shared
/// cooldown group id (if any).
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ItemMeta {
    class: ItemClass,
    abilities: &'static [WarcraftObjectId],
    cooldown_id: Option<WarcraftObjectId>,
}

impl ItemMeta {
    pub fn new(
        class: ItemClass,
        abilities: &'static [WarcraftObjectId],
        cooldown_id: Option<WarcraftObjectId>,
    ) -> Self {
        Self {
            class,
            abilities,
            cooldown_id,
        }
    }

    pub fn cooldown_id(&self) -> Option<WarcraftObjectId> {
        self.cooldown_id
    }

    pub fn abilities(&self) -> &'static [WarcraftObjectId] {
        self.abilities
    }

    pub fn class(&self) -> &ItemClass {
        &self.class
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for ItemMeta {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for ItemMeta {}
