//! [`UnitAttack`]: a unit's weapon — damage, range, cooldown, and typing.

use crate::domain::combat::{AttackType, WeaponType};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct UnitAttack {
    damage_min: u32,
    damage_max: u32,
    range: u32,
    cooldown_seconds: f32,
    attack_type: AttackType,
    weapon_type: WeaponType,
}

impl UnitAttack {
    pub const fn new(
        damage_min: u32,
        damage_max: u32,
        range: u32,
        cooldown_seconds: f32,
        attack_type: AttackType,
        weapon_type: WeaponType,
    ) -> Self {
        Self {
            damage_min,
            damage_max,
            range,
            cooldown_seconds,
            attack_type,
            weapon_type,
        }
    }

    pub fn damage_min(&self) -> u32 {
        self.damage_min
    }

    pub fn damage_max(&self) -> u32 {
        self.damage_max
    }

    pub fn range(&self) -> u32 {
        self.range
    }

    pub fn cooldown_seconds(&self) -> f32 {
        self.cooldown_seconds
    }

    pub fn attack_type(&self) -> AttackType {
        self.attack_type
    }

    pub fn weapon_type(&self) -> WeaponType {
        self.weapon_type
    }

    pub fn targets_ground(&self) -> bool {
        self.weapon_type.targets_ground()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_attack_targets_ground_reflects_weapon_type() {
        let artillery_attack =
            UnitAttack::new(50, 60, 1000, 2.0, AttackType::Siege, WeaponType::Artillery);
        let normal_attack =
            UnitAttack::new(10, 12, 90, 1.5, AttackType::Normal, WeaponType::Normal);
        assert!(artillery_attack.targets_ground());
        assert!(!normal_attack.targets_ground());
    }
}
