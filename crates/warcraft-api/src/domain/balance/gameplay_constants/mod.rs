//! [`GameplayConstants`]: the global gameplay balance constants — attribute
//! bonuses, the damage matrix, and the hero level cap.

use crate::domain::balance::agility_bonuses::AgilityBonuses;
use crate::domain::balance::damage_effectiveness::DamageEffectiveness;
use crate::domain::balance::damage_matrix::DamageMatrix;
use crate::domain::balance::intelligence_bonuses::IntelligenceBonuses;
use crate::domain::balance::strength_bonuses::StrengthBonuses;
use crate::domain::combat::AttackType;
use crate::domain::quantity::Multiplier;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameplayConstants {
    // Defaults below mirror the standard WC3 Reforged values from
    // war3.w3mod:units/miscgame.txt; used only when extraction is missing
    // the field, so the runtime never sees an all-zero `GameplayConstants`.
    strength_bonuses: StrengthBonuses,
    intelligence_bonuses: IntelligenceBonuses,
    agility_bonuses: AgilityBonuses,
    max_hero_level: u32,
    damage_matrix: DamageMatrix,
}

impl GameplayConstants {
    pub const fn new(
        strength_bonuses: StrengthBonuses,
        intelligence_bonuses: IntelligenceBonuses,
        agility_bonuses: AgilityBonuses,
        max_hero_level: u32,
        damage_matrix: DamageMatrix,
    ) -> Self {
        Self {
            strength_bonuses,
            intelligence_bonuses,
            agility_bonuses,
            max_hero_level,
            damage_matrix,
        }
    }

    pub fn strength_bonuses(&self) -> StrengthBonuses {
        self.strength_bonuses
    }

    pub fn intelligence_bonuses(&self) -> IntelligenceBonuses {
        self.intelligence_bonuses
    }

    pub fn agility_bonuses(&self) -> AgilityBonuses {
        self.agility_bonuses
    }

    pub fn damage_matrix(&self) -> DamageMatrix {
        self.damage_matrix
    }

    pub fn damage_effectiveness(&self, attack_type: AttackType) -> DamageEffectiveness {
        self.damage_matrix.effectiveness(attack_type)
    }

    pub fn str_attack_bonus(&self) -> Multiplier {
        self.strength_bonuses.attack_bonus()
    }

    pub fn str_hit_point_bonus(&self) -> u32 {
        self.strength_bonuses.hit_point_bonus()
    }

    pub fn str_regen_bonus(&self) -> Multiplier {
        self.strength_bonuses.regen_bonus()
    }

    pub fn int_mana_bonus(&self) -> u32 {
        self.intelligence_bonuses.mana_bonus()
    }

    pub fn int_regen_bonus(&self) -> Multiplier {
        self.intelligence_bonuses.regen_bonus()
    }

    pub fn agi_defense_bonus(&self) -> Multiplier {
        self.agility_bonuses.defense_bonus()
    }

    pub fn agi_attack_speed_bonus(&self) -> Multiplier {
        self.agility_bonuses.attack_speed_bonus()
    }

    pub fn max_hero_level(&self) -> u32 {
        self.max_hero_level
    }
}

/// Build the eight per-defense multipliers from their milli values.
const fn effectiveness(milli: [u32; 8]) -> DamageEffectiveness {
    DamageEffectiveness::new([
        Multiplier::from_milli(milli[0]),
        Multiplier::from_milli(milli[1]),
        Multiplier::from_milli(milli[2]),
        Multiplier::from_milli(milli[3]),
        Multiplier::from_milli(milli[4]),
        Multiplier::from_milli(milli[5]),
        Multiplier::from_milli(milli[6]),
        Multiplier::from_milli(milli[7]),
    ])
}

impl Default for GameplayConstants {
    fn default() -> Self {
        // SMALL, MEDIUM, LARGE, FORT, NORMAL, HERO, DIVINE, NONE — matches
        // miscgame.txt DamageBonus* line order. Values in milli (×1000).
        let damage_matrix = DamageMatrix::new(
            effectiveness([1000, 1500, 1000, 700, 1000, 1000, 50, 1000]),
            effectiveness([2000, 750, 1000, 350, 1000, 500, 50, 1500]),
            effectiveness([1000, 500, 1000, 1500, 1000, 500, 50, 1500]),
            effectiveness([1250, 750, 2000, 350, 1000, 500, 50, 1000]),
            effectiveness([1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000]),
            effectiveness([1000, 1000, 1000, 1000, 1000, 700, 50, 1000]),
            effectiveness([1000, 1000, 1000, 500, 1000, 1000, 50, 1000]),
        );
        let strength_bonuses =
            StrengthBonuses::new(Multiplier::from_milli(1000), 25, Multiplier::from_milli(50));
        let intelligence_bonuses = IntelligenceBonuses::new(15, Multiplier::from_milli(50));
        let agility_bonuses =
            AgilityBonuses::new(Multiplier::from_milli(300), Multiplier::from_milli(20));
        Self::new(
            strength_bonuses,
            intelligence_bonuses,
            agility_bonuses,
            10,
            damage_matrix,
        )
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for GameplayConstants {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for GameplayConstants {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gameplay_constants_default_has_reasonable_max_hero_level() {
        assert_eq!(GameplayConstants::default().max_hero_level(), 10);
    }

    #[test]
    fn gameplay_constants_default_str_per_hp_is_nonzero() {
        assert!(GameplayConstants::default().str_hit_point_bonus() > 0);
    }

    #[test]
    fn damage_matrix_chaos_is_effective_against_all_armor() {
        let chaos = GameplayConstants::default().damage_effectiveness(AttackType::Chaos);
        for multiplier in chaos.multipliers() {
            assert_eq!(*multiplier, Multiplier::from_milli(1000));
        }
    }
}
