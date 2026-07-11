//! [`DamageEffectiveness`]: the eight per-defense-type damage multipliers of one
//! attack type.

use crate::domain::combat::DefenseType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DamageEffectiveness {
    // Eight multipliers, one per defense type, in the order returned by
    // `DefenseType::all()`: Light, Medium, Heavy, Fortified, Normal, Hero,
    // Divine, Unarmored. Sourced from `DamageBonus*` lines in
    // `war3.w3mod:units/miscgame.txt`.
    multipliers: [f32; 8],
}

impl DamageEffectiveness {
    pub const fn new(multipliers: [f32; 8]) -> Self {
        Self { multipliers }
    }

    pub fn against(&self, defense_type: DefenseType) -> f32 {
        let defense_types = DefenseType::all();
        let mut iterator_index = 0;
        while iterator_index < defense_types.len() {
            if defense_types[iterator_index] == defense_type {
                return self.multipliers[iterator_index];
            }
            iterator_index += 1;
        }
        1.0
    }

    pub fn multipliers(&self) -> &[f32; 8] {
        &self.multipliers
    }
}

impl Default for DamageEffectiveness {
    fn default() -> Self {
        Self::new([1.0; 8])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_effectiveness_against_returns_correct_multiplier() {
        let all_twos = DamageEffectiveness::new([2.0; 8]);
        assert_eq!(all_twos.against(DefenseType::Light), 2.0);
        assert_eq!(all_twos.against(DefenseType::Divine), 2.0);
    }

    #[test]
    fn damage_effectiveness_default_is_all_ones() {
        let effectiveness = DamageEffectiveness::default();
        for &multiplier in effectiveness.multipliers() {
            assert_eq!(multiplier, 1.0);
        }
    }
}
