//! [`AttributeBase`]: a hero's level-1 strength / agility / intelligence.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttributeBase {
    strength: u32,
    agility: u32,
    intelligence: u32,
}

impl AttributeBase {
    pub const fn new(strength: u32, agility: u32, intelligence: u32) -> Self {
        Self {
            strength,
            agility,
            intelligence,
        }
    }

    pub fn strength(&self) -> u32 {
        self.strength
    }

    pub fn agility(&self) -> u32 {
        self.agility
    }

    pub fn intelligence(&self) -> u32 {
        self.intelligence
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for AttributeBase {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for AttributeBase {}
