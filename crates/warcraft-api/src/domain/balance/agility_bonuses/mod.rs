//! [`AgilityBonuses`]: the per-point bonuses a hero's Agility attribute grants.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AgilityBonuses {
    defense_bonus: f32,
    attack_speed_bonus: f32,
}

impl AgilityBonuses {
    pub const fn new(defense_bonus: f32, attack_speed_bonus: f32) -> Self {
        Self {
            defense_bonus,
            attack_speed_bonus,
        }
    }

    pub fn defense_bonus(&self) -> f32 {
        self.defense_bonus
    }

    pub fn attack_speed_bonus(&self) -> f32 {
        self.attack_speed_bonus
    }
}
