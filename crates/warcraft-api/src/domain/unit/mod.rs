//! Unit domain concept: the unit taxonomy plus its production / flags / regen
//! value objects and the (still float-bearing) combat, attack, and meta types.
//!
//! `UnitMeta`, `UnitCombat`, and `UnitAttack` still carry `f32` fields and so
//! are not yet `Eq`/`ValueObject`-marked — that lands with the fixed-point
//! conversion (slice 3).

pub(crate) mod attack;
pub(crate) mod combat;
pub(crate) mod flags;
pub(crate) mod hero;
pub(crate) mod kind;
pub(crate) mod meta;
pub(crate) mod mode;
pub(crate) mod production;
pub(crate) mod regen;

pub use attack::UnitAttack;
pub use combat::UnitCombat;
pub use flags::UnitFlags;
pub use kind::UnitKind;
pub use meta::UnitMeta;
pub use mode::UnitMode;
pub use production::UnitProduction;
pub use regen::RegenType;
