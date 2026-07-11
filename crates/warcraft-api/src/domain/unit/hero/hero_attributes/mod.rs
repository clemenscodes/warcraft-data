//! [`HeroAttributes`]: the composite attribute block of a hero unit.

use crate::domain::unit::hero::attribute_base::AttributeBase;
use crate::domain::unit::hero::attribute_growth::AttributeGrowth;
use crate::domain::unit::hero::mana_pool::ManaPool;
use crate::domain::unit::hero::primary_attribute::PrimaryAttribute;

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
        self.mana_pool.mana()
    }

    pub fn mana_regen(&self) -> f32 {
        self.mana_pool.mana_regen()
    }

    pub fn strength(&self) -> u32 {
        self.base.strength()
    }

    pub fn agility(&self) -> u32 {
        self.base.agility()
    }

    pub fn intelligence(&self) -> u32 {
        self.base.intelligence()
    }

    pub fn primary(&self) -> PrimaryAttribute {
        self.primary
    }

    pub fn strength_per_level(&self) -> f32 {
        self.growth.strength_per_level()
    }

    pub fn agility_per_level(&self) -> f32 {
        self.growth.agility_per_level()
    }

    pub fn intelligence_per_level(&self) -> f32 {
        self.growth.intelligence_per_level()
    }
}
