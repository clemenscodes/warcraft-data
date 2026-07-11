//! [`UnitCombat`]: a unit's defensive stats plus its optional attack and mana.

use crate::domain::combat::DefenseType;
use crate::domain::quantity::{Armor, RegenRate};
use crate::domain::unit::attack::UnitAttack;
use crate::domain::unit::hero::ManaPool;
use crate::domain::unit::regen::RegenType;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitCombat {
    hit_points: u32,
    hit_points_regen: RegenRate,
    regen_type: RegenType,
    armor: Armor,
    defense_type: DefenseType,
    attack: Option<UnitAttack>,
    mana_pool: Option<ManaPool>,
}

impl UnitCombat {
    pub const EMPTY: UnitCombat = UnitCombat {
        hit_points: 0,
        hit_points_regen: RegenRate::from_milli(0),
        regen_type: RegenType::Always,
        armor: Armor::from_milli(0),
        defense_type: DefenseType::Unarmored,
        attack: None,
        mana_pool: None,
    };

    pub const fn new(
        hit_points: u32,
        hit_points_regen: RegenRate,
        regen_type: RegenType,
        armor: Armor,
        defense_type: DefenseType,
        attack: Option<UnitAttack>,
    ) -> Self {
        Self {
            hit_points,
            hit_points_regen,
            regen_type,
            armor,
            defense_type,
            attack,
            mana_pool: None,
        }
    }

    pub const fn with_mana_pool(mut self, mana_pool: ManaPool) -> Self {
        self.mana_pool = Some(mana_pool);
        self
    }

    pub fn hit_points(&self) -> u32 {
        self.hit_points
    }

    pub fn hit_points_regen(&self) -> RegenRate {
        self.hit_points_regen
    }

    pub fn regen_type(&self) -> RegenType {
        self.regen_type
    }

    pub fn armor(&self) -> Armor {
        self.armor
    }

    pub fn defense_type(&self) -> DefenseType {
        self.defense_type
    }

    pub fn attack(&self) -> Option<&UnitAttack> {
        self.attack.as_ref()
    }

    pub fn mana_pool(&self) -> Option<ManaPool> {
        self.mana_pool
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for UnitCombat {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for UnitCombat {}
