//! [`DamageMatrix`]: the per-attack-type table of [`DamageEffectiveness`].

use crate::domain::balance::damage_effectiveness::DamageEffectiveness;
use crate::domain::combat::AttackType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DamageMatrix {
    normal: DamageEffectiveness,
    pierce: DamageEffectiveness,
    siege: DamageEffectiveness,
    magic: DamageEffectiveness,
    chaos: DamageEffectiveness,
    spells: DamageEffectiveness,
    hero: DamageEffectiveness,
}

impl DamageMatrix {
    pub const fn new(
        normal: DamageEffectiveness,
        pierce: DamageEffectiveness,
        siege: DamageEffectiveness,
        magic: DamageEffectiveness,
        chaos: DamageEffectiveness,
        spells: DamageEffectiveness,
        hero: DamageEffectiveness,
    ) -> Self {
        Self {
            normal,
            pierce,
            siege,
            magic,
            chaos,
            spells,
            hero,
        }
    }

    pub fn effectiveness(&self, attack_type: AttackType) -> DamageEffectiveness {
        match attack_type {
            AttackType::Normal => self.normal,
            AttackType::Pierce => self.pierce,
            AttackType::Siege => self.siege,
            AttackType::Magic => self.magic,
            AttackType::Chaos => self.chaos,
            AttackType::Spells => self.spells,
            AttackType::Hero => self.hero,
            AttackType::Unknown => DamageEffectiveness::new([1.0; 8]),
        }
    }

    pub fn normal(&self) -> DamageEffectiveness {
        self.normal
    }

    pub fn pierce(&self) -> DamageEffectiveness {
        self.pierce
    }

    pub fn siege(&self) -> DamageEffectiveness {
        self.siege
    }

    pub fn magic(&self) -> DamageEffectiveness {
        self.magic
    }

    pub fn chaos(&self) -> DamageEffectiveness {
        self.chaos
    }

    pub fn spells(&self) -> DamageEffectiveness {
        self.spells
    }

    pub fn hero(&self) -> DamageEffectiveness {
        self.hero
    }
}
