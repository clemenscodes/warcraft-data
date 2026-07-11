//! Combat statistics: a unit's fighting figures as strongly-typed domain values.
//!
//! [`UnitStatistics`] is the aggregate a statistics card receives — every figure
//! is its own wrapper type ([`HitPoints`], [`ArmorFigure`], [`Mana`], …) so a
//! consumer reads typed values, never a bag of pre-formatted strings. The derived
//! figures (effective hit points, damage per second) and hero-level attribute
//! scaling are computed here as pure domain arithmetic; the evasion *chance* a
//! unit fields is resolved from the database in the application layer
//! ([`crate::UnitApi::evasion`]) and fed in.
//!
//! These figures are `f32`-bearing display values, deliberately distinct from the
//! `Eq` fixed-point [`crate::Armor`]/[`crate::Cooldown`] quantities that store the
//! source-of-truth data — hence `ArmorFigure` rather than `Armor`.

pub(crate) mod attack;
pub(crate) mod hero;
pub(crate) mod matchup;
pub(crate) mod unit_statistics;
pub(crate) mod values;

pub use attack::AttackStatistics;
pub use hero::{AttributeStatistic, HeroStatistics};
pub use matchup::{Matchup, MatchupStrength};
pub use unit_statistics::UnitStatistics;
pub use values::{
    ArmorFigure, AttackRange, AttackSpeed, DamagePerSecond, DamageRange, EffectiveHitPoints,
    Evasion, Gain, HitPoints, HitPointsRegen, Mana, ManaRegen,
};
