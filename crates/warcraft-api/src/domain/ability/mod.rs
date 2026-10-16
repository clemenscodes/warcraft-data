//! Ability domain concept: the ability's kind-specific metadata value object
//! and the hero talents that replace an ability on its button.
//!
//! `AbilityMeta` still carries `f32` fields (evasion chances) and so is not yet
//! `Eq`/`ValueObject`-marked — that lands with the fixed-point conversion
//! (slice 3).

pub(crate) mod meta;
pub(crate) mod talent;

pub use meta::{AbilityMeta, MAX_ABILITY_LEVEL};
pub use talent::TalentVariant;
