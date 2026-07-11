//! [`AttributeGrowth`]: a hero's per-level attribute gain.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributeGrowth {
    strength_per_level: f32,
    agility_per_level: f32,
    intelligence_per_level: f32,
}

impl AttributeGrowth {
    pub const fn new(
        strength_per_level: f32,
        agility_per_level: f32,
        intelligence_per_level: f32,
    ) -> Self {
        Self {
            strength_per_level,
            agility_per_level,
            intelligence_per_level,
        }
    }

    pub fn strength_per_level(&self) -> f32 {
        self.strength_per_level
    }

    pub fn agility_per_level(&self) -> f32 {
        self.agility_per_level
    }

    pub fn intelligence_per_level(&self) -> f32 {
        self.intelligence_per_level
    }
}
