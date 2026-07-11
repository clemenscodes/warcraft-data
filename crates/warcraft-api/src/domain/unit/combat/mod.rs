//! [`UnitCombat`]: a unit's defensive stats plus its optional attack and mana.

use crate::domain::combat::DefenseType;
use crate::domain::unit::attack::UnitAttack;
use crate::domain::unit::hero::ManaPool;
use crate::domain::unit::regen::RegenType;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct UnitCombat {
    hit_points: u32,
    hit_points_regen: f32,
    regen_type: RegenType,
    armor: f32,
    defense_type: DefenseType,
    attack: Option<UnitAttack>,
    mana_pool: Option<ManaPool>,
}

impl UnitCombat {
    pub const EMPTY: UnitCombat = UnitCombat {
        hit_points: 0,
        hit_points_regen: 0.0,
        regen_type: RegenType::Always,
        armor: 0.0,
        defense_type: DefenseType::Unarmored,
        attack: None,
        mana_pool: None,
    };

    pub const fn new(
        hit_points: u32,
        hit_points_regen: f32,
        regen_type: RegenType,
        armor: f32,
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

    pub fn hit_points_regen(&self) -> f32 {
        self.hit_points_regen
    }

    pub fn regen_type(&self) -> RegenType {
        self.regen_type
    }

    pub fn armor(&self) -> f32 {
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
