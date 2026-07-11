//! Balance domain concept: the global gameplay constants plus the damage and
//! attribute-bonus value objects they aggregate.
//!
//! These types still carry `f32` fields and so are not yet `ValueObject`-marked
//! — that lands with the fixed-point conversion (slice 3).

pub(crate) mod agility_bonuses;
pub(crate) mod damage_effectiveness;
pub(crate) mod damage_matrix;
pub(crate) mod gameplay_constants;
pub(crate) mod intelligence_bonuses;
pub(crate) mod strength_bonuses;

pub use agility_bonuses::AgilityBonuses;
pub use damage_effectiveness::DamageEffectiveness;
pub use damage_matrix::DamageMatrix;
pub use gameplay_constants::GameplayConstants;
pub use intelligence_bonuses::IntelligenceBonuses;
pub use strength_bonuses::StrengthBonuses;
