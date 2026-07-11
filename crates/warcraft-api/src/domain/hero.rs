//! Hero attributes: primary attribute, base stats, per-level growth, mana pool
//! and the composite hero attribute block. `ManaPool` lives here (rather than in
//! `unit`) because both hero attributes and unit combat reference it, and this
//! placement keeps the dependency acyclic (`unit` → `hero`).
//!
//! NOTE: the float-bearing types (`ManaPool`, `AttributeGrowth`, `HeroAttributes`)
//! carry no `ValueObject` marker yet — that follows once their rates become
//! fixed-point quantity VOs (slice 3).

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryAttribute {
    #[default]
    Strength,
    Agility,
    Intelligence,
}

impl PrimaryAttribute {
    pub fn parse(raw: &str) -> Option<PrimaryAttribute> {
        let normalized = raw.trim().to_ascii_uppercase();
        match normalized.as_str() {
            "STR" => Some(PrimaryAttribute::Strength),
            "AGI" => Some(PrimaryAttribute::Agility),
            "INT" => Some(PrimaryAttribute::Intelligence),
            _ => None,
        }
    }
}

impl std::fmt::Display for PrimaryAttribute {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            PrimaryAttribute::Strength => "Strength",
            PrimaryAttribute::Agility => "Agility",
            PrimaryAttribute::Intelligence => "Intelligence",
        };
        formatter.write_str(label)
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeroAttributes {
    mana_pool: ManaPool,
    base: AttributeBase,
    growth: AttributeGrowth,
    primary: PrimaryAttribute,
}

impl HeroAttributes {
    pub const fn new(
        mana_pool: ManaPool,
        base: AttributeBase,
        growth: AttributeGrowth,
        primary: PrimaryAttribute,
    ) -> Self {
        Self {
            mana_pool,
            base,
            growth,
            primary,
        }
    }

    pub fn mana_pool(&self) -> ManaPool {
        self.mana_pool
    }

    pub fn base(&self) -> AttributeBase {
        self.base
    }

    pub fn growth(&self) -> AttributeGrowth {
        self.growth
    }

    pub fn mana(&self) -> u32 {
        self.mana_pool.mana
    }

    pub fn mana_regen(&self) -> f32 {
        self.mana_pool.mana_regen
    }

    pub fn strength(&self) -> u32 {
        self.base.strength
    }

    pub fn agility(&self) -> u32 {
        self.base.agility
    }

    pub fn intelligence(&self) -> u32 {
        self.base.intelligence
    }

    pub fn primary(&self) -> PrimaryAttribute {
        self.primary
    }

    pub fn strength_per_level(&self) -> f32 {
        self.growth.strength_per_level
    }

    pub fn agility_per_level(&self) -> f32 {
        self.growth.agility_per_level
    }

    pub fn intelligence_per_level(&self) -> f32 {
        self.growth.intelligence_per_level
    }
}

// DDD roles (float-free types only for now).
impl ddd::Layered for PrimaryAttribute {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for PrimaryAttribute {}

impl ddd::Layered for AttributeBase {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for AttributeBase {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_attribute_parse_case_insensitive() {
        assert_eq!(
            PrimaryAttribute::parse("str"),
            Some(PrimaryAttribute::Strength)
        );
        assert_eq!(
            PrimaryAttribute::parse("AGI"),
            Some(PrimaryAttribute::Agility)
        );
        assert_eq!(
            PrimaryAttribute::parse("int"),
            Some(PrimaryAttribute::Intelligence)
        );
    }

    #[test]
    fn primary_attribute_parse_unknown_is_none() {
        assert_eq!(PrimaryAttribute::parse("xyz"), None);
    }

    #[test]
    fn primary_attribute_display_is_full_name() {
        assert_eq!(PrimaryAttribute::Strength.to_string(), "Strength");
        assert_eq!(PrimaryAttribute::Agility.to_string(), "Agility");
        assert_eq!(PrimaryAttribute::Intelligence.to_string(), "Intelligence");
    }
}
