//! [`StrengthBonuses`]: the per-point bonuses a hero's Strength attribute grants.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrengthBonuses {
    attack_bonus: f32,
    hit_point_bonus: u32,
    regen_bonus: f32,
}

impl StrengthBonuses {
    pub const fn new(attack_bonus: f32, hit_point_bonus: u32, regen_bonus: f32) -> Self {
        Self {
            attack_bonus,
            hit_point_bonus,
            regen_bonus,
        }
    }

    pub fn attack_bonus(&self) -> f32 {
        self.attack_bonus
    }

    pub fn hit_point_bonus(&self) -> u32 {
        self.hit_point_bonus
    }

    pub fn regen_bonus(&self) -> f32 {
        self.regen_bonus
    }
}
