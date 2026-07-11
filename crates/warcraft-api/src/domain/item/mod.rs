//! Item domain concept: an item's classification and its metadata.

pub(crate) mod class;
pub(crate) mod meta;

pub use class::ItemClass;
pub use meta::ItemMeta;
