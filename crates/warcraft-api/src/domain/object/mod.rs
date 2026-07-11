//! Object domain concept: the `WarcraftObject` aggregate root, its kind and
//! meta discriminants, and its display text.

pub(crate) mod aggregate;
pub(crate) mod kind;
pub(crate) mod meta;
pub(crate) mod text;

pub use aggregate::WarcraftObject;
pub use kind::WarcraftObjectKind;
pub use meta::WarcraftObjectMeta;
pub use text::{Description, Tip, WarcraftColorCodes, WarcraftObjectText};
