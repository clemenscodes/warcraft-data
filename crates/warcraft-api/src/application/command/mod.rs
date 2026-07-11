//! The `command` domain concept's application surface: [`CommandApi`] and the
//! [`CommandView`](crate::CommandView) read model it produces. A command is a
//! standalone in-game action (hotkey-bindable), not tied to any unit.

pub(crate) mod api;

pub use api::CommandApi;
