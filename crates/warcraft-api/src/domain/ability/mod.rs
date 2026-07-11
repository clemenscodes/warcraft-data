//! Ability domain concept: the ability's kind-specific metadata value object.
//!
//! `AbilityMeta` still carries `f32` fields (evasion chances) and so is not yet
//! `Eq`/`ValueObject`-marked — that lands with the fixed-point conversion
//! (slice 3).

pub(crate) mod meta;

pub use meta::AbilityMeta;
