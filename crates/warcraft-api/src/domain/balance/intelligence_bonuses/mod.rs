//! [`IntelligenceBonuses`]: the per-point bonuses a hero's Intelligence grants.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntelligenceBonuses {
    mana_bonus: u32,
    regen_bonus: f32,
}

impl IntelligenceBonuses {
    pub const fn new(mana_bonus: u32, regen_bonus: f32) -> Self {
        Self {
            mana_bonus,
            regen_bonus,
        }
    }

    pub fn mana_bonus(&self) -> u32 {
        self.mana_bonus
    }

    pub fn regen_bonus(&self) -> f32 {
        self.regen_bonus
    }
}
