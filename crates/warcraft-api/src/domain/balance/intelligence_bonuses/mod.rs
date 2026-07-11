//! [`IntelligenceBonuses`]: the per-point bonuses a hero's Intelligence grants.

use crate::domain::quantity::Multiplier;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntelligenceBonuses {
    mana_bonus: u32,
    regen_bonus: Multiplier,
}

impl IntelligenceBonuses {
    pub const fn new(mana_bonus: u32, regen_bonus: Multiplier) -> Self {
        Self {
            mana_bonus,
            regen_bonus,
        }
    }

    pub fn mana_bonus(&self) -> u32 {
        self.mana_bonus
    }

    pub fn regen_bonus(&self) -> Multiplier {
        self.regen_bonus
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for IntelligenceBonuses {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for IntelligenceBonuses {}
