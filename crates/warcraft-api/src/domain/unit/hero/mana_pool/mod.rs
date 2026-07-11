//! [`ManaPool`]: a unit's mana capacity and regeneration. Referenced by both
//! hero attributes and unit combat.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ManaPool {
    mana: u32,
    mana_regen: f32,
}

impl ManaPool {
    pub const fn new(mana: u32, mana_regen: f32) -> Self {
        Self { mana, mana_regen }
    }

    pub fn mana(&self) -> u32 {
        self.mana
    }

    pub fn mana_regen(&self) -> f32 {
        self.mana_regen
    }
}
