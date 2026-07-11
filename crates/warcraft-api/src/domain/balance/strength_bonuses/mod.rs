//! [`StrengthBonuses`]: the per-point bonuses a hero's Strength attribute grants.

use crate::domain::quantity::Multiplier;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrengthBonuses {
    attack_bonus: Multiplier,
    hit_point_bonus: u32,
    regen_bonus: Multiplier,
}

impl StrengthBonuses {
    pub const fn new(
        attack_bonus: Multiplier,
        hit_point_bonus: u32,
        regen_bonus: Multiplier,
    ) -> Self {
        Self {
            attack_bonus,
            hit_point_bonus,
            regen_bonus,
        }
    }

    pub fn attack_bonus(&self) -> Multiplier {
        self.attack_bonus
    }

    pub fn hit_point_bonus(&self) -> u32 {
        self.hit_point_bonus
    }

    pub fn regen_bonus(&self) -> Multiplier {
        self.regen_bonus
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for StrengthBonuses {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for StrengthBonuses {}
