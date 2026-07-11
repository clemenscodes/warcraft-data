//! Fixed-point quantity value objects. The game's stats are fractional but never
//! irrational; storing them as `f32` in the domain would forfeit `Eq`/`Hash`
//! (because of `NaN`). These integer-backed value objects keep exact,
//! equality-comparable values on a milli scale (×1000), or permille for chances.
//! The `f32` → integer translation happens once, in the extractor, when it emits
//! the generated database — the domain only ever sees the integer constructors.

pub(crate) mod armor;
pub(crate) mod chance;
pub(crate) mod cooldown;
pub(crate) mod multiplier;
pub(crate) mod regen_rate;
pub(crate) mod stat_growth;

pub use armor::Armor;
pub use chance::Chance;
pub use cooldown::Cooldown;
pub use multiplier::Multiplier;
pub use regen_rate::RegenRate;
pub use stat_growth::StatGrowth;
