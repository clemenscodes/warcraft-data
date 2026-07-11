//! Hero sub-concept of `unit`: a hero unit's primary attribute, base stats,
//! per-level growth, and mana pool.
//!
//! The float-bearing types (`ManaPool`, `AttributeGrowth`, `HeroAttributes`)
//! carry no `ValueObject` marker yet — that follows with the fixed-point
//! conversion (slice 3).

pub(crate) mod attribute_base;
pub(crate) mod attribute_growth;
pub(crate) mod hero_attributes;
pub(crate) mod mana_pool;
pub(crate) mod primary_attribute;

pub use attribute_base::AttributeBase;
pub use attribute_growth::AttributeGrowth;
pub use hero_attributes::HeroAttributes;
pub use mana_pool::ManaPool;
pub use primary_attribute::PrimaryAttribute;
