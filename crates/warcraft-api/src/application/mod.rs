//! Application layer: the public entry point. `WarcraftApi` is the single
//! `ApplicationService` through which every consumer reads the game database;
//! the backing store and its type are never exposed.

pub(crate) mod ability;
pub(crate) mod api;
pub(crate) mod command;
pub(crate) mod unit;
pub(crate) mod view;
