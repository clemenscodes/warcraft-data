//! The `unit` domain concept's application surface: [`UnitApi`] and the
//! [`UnitView`](crate::UnitView) read model it produces.

pub(crate) mod api;
pub(crate) mod variant;

pub use api::UnitApi;
