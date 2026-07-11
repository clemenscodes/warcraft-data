//! Combat taxonomy: the attack / weapon / defense type value objects. Shared
//! vocabulary — consumed both by unit combat stats and by the balance damage
//! matrix.

pub(crate) mod attack_type;
pub(crate) mod defense_type;
pub(crate) mod weapon_type;

pub use attack_type::AttackType;
pub use defense_type::DefenseType;
pub use weapon_type::WeaponType;
