//! Command domain concept: metadata for command-card command buttons and the
//! display-label derivation for command ids.

pub(crate) mod command_label;
pub(crate) mod meta;

pub use command_label::CommandLabel;
pub use meta::CommandMeta;
