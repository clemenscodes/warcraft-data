//! Upgrade domain concept: upgrade metadata and upgrade-driven unit swaps.

pub(crate) mod meta;
pub(crate) mod swap;

pub use meta::UpgradeMeta;
pub use swap::UnitUpgradeSwap;
