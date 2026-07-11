//! [`DamageEffectiveness`]: the eight per-defense-type damage multipliers of one
//! attack type.

use crate::domain::combat::DefenseType;
use crate::domain::quantity::Multiplier;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DamageEffectiveness {
    // Eight multipliers, one per defense type, in the order returned by
    // `DefenseType::all()`: Light, Medium, Heavy, Fortified, Normal, Hero,
    // Divine, Unarmored. Sourced from `DamageBonus*` lines in
    // `war3.w3mod:units/miscgame.txt`.
    multipliers: [Multiplier; 8],
}

impl DamageEffectiveness {
    pub const fn new(multipliers: [Multiplier; 8]) -> Self {
        Self { multipliers }
    }

    pub fn against(&self, defense_type: DefenseType) -> Multiplier {
        let defense_types = DefenseType::all();
        let mut iterator_index = 0;
        while iterator_index < defense_types.len() {
            if defense_types[iterator_index] == defense_type {
                return self.multipliers[iterator_index];
            }
            iterator_index += 1;
        }
        Multiplier::from_milli(1000)
    }

    pub fn multipliers(&self) -> &[Multiplier; 8] {
        &self.multipliers
    }
}

impl Default for DamageEffectiveness {
    fn default() -> Self {
        Self::new([Multiplier::from_milli(1000); 8])
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for DamageEffectiveness {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for DamageEffectiveness {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_effectiveness_against_returns_correct_multiplier() {
        let all_twos = DamageEffectiveness::new([Multiplier::from_milli(2000); 8]);
        assert_eq!(
            all_twos.against(DefenseType::Light),
            Multiplier::from_milli(2000)
        );
        assert_eq!(
            all_twos.against(DefenseType::Divine),
            Multiplier::from_milli(2000)
        );
    }

    #[test]
    fn damage_effectiveness_default_is_all_ones() {
        let effectiveness = DamageEffectiveness::default();
        for multiplier in effectiveness.multipliers() {
            assert_eq!(*multiplier, Multiplier::from_milli(1000));
        }
    }
}
