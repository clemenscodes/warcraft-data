//! [`AttributeGrowth`]: a hero's per-level attribute gain.

use crate::domain::quantity::StatGrowth;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttributeGrowth {
    strength_per_level: StatGrowth,
    agility_per_level: StatGrowth,
    intelligence_per_level: StatGrowth,
}

impl AttributeGrowth {
    pub const fn new(
        strength_per_level: StatGrowth,
        agility_per_level: StatGrowth,
        intelligence_per_level: StatGrowth,
    ) -> Self {
        Self {
            strength_per_level,
            agility_per_level,
            intelligence_per_level,
        }
    }

    pub fn strength_per_level(&self) -> StatGrowth {
        self.strength_per_level
    }

    pub fn agility_per_level(&self) -> StatGrowth {
        self.agility_per_level
    }

    pub fn intelligence_per_level(&self) -> StatGrowth {
        self.intelligence_per_level
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for AttributeGrowth {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for AttributeGrowth {}
