//! [`ManaPool`]: a unit's mana capacity and regeneration. Referenced by both
//! hero attributes and unit combat.

use crate::domain::quantity::RegenRate;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManaPool {
    mana: u32,
    mana_regen: RegenRate,
}

impl ManaPool {
    pub const fn new(mana: u32, mana_regen: RegenRate) -> Self {
        Self { mana, mana_regen }
    }

    pub fn mana(&self) -> u32 {
        self.mana
    }

    pub fn mana_regen(&self) -> RegenRate {
        self.mana_regen
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for ManaPool {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for ManaPool {}
