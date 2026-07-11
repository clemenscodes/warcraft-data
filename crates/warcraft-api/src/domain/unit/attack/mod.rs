//! [`UnitAttack`]: a unit's weapon — damage, range, cooldown, and typing.

use crate::domain::combat::{AttackType, WeaponType};
use crate::domain::quantity::Cooldown;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitAttack {
    damage_min: u32,
    damage_max: u32,
    range: u32,
    cooldown: Cooldown,
    attack_type: AttackType,
    weapon_type: WeaponType,
}

impl UnitAttack {
    pub const fn new(
        damage_min: u32,
        damage_max: u32,
        range: u32,
        cooldown: Cooldown,
        attack_type: AttackType,
        weapon_type: WeaponType,
    ) -> Self {
        Self {
            damage_min,
            damage_max,
            range,
            cooldown,
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

    pub fn cooldown(&self) -> Cooldown {
        self.cooldown
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

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for UnitAttack {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for UnitAttack {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_attack_targets_ground_reflects_weapon_type() {
        let artillery_attack = UnitAttack::new(
            50,
            60,
            1000,
            Cooldown::from_millis(2000),
            AttackType::Siege,
            WeaponType::Artillery,
        );
        let normal_attack = UnitAttack::new(
            10,
            12,
            90,
            Cooldown::from_millis(1500),
            AttackType::Normal,
            WeaponType::Normal,
        );
        assert!(artillery_attack.targets_ground());
        assert!(!normal_attack.targets_ground());
    }
}
