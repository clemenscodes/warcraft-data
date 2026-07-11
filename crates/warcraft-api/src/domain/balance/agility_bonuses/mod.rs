//! [`AgilityBonuses`]: the per-point bonuses a hero's Agility attribute grants.

use crate::domain::quantity::Multiplier;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgilityBonuses {
    defense_bonus: Multiplier,
    attack_speed_bonus: Multiplier,
}

impl AgilityBonuses {
    pub const fn new(defense_bonus: Multiplier, attack_speed_bonus: Multiplier) -> Self {
        Self {
            defense_bonus,
            attack_speed_bonus,
        }
    }

    pub fn defense_bonus(&self) -> Multiplier {
        self.defense_bonus
    }

    pub fn attack_speed_bonus(&self) -> Multiplier {
        self.attack_speed_bonus
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for AgilityBonuses {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for AgilityBonuses {}
