//! Domain layer: the game's modeled vocabulary — Value Objects, Entities and
//! Identifiers. Every type here lives in `ddd::DomainLayer` and is marked with
//! its DDD role. Split by domain concern, one concern per module.

pub(crate) mod ability;
pub(crate) mod balance;
pub(crate) mod building;
pub(crate) mod catalog;
pub(crate) mod combat;
pub(crate) mod command;
pub(crate) mod grid;
pub(crate) mod identity;
pub(crate) mod item;
pub(crate) mod keybind;
pub(crate) mod object;
pub(crate) mod player;
pub(crate) mod race;
pub(crate) mod unit;
pub(crate) mod upgrade;
pub(crate) mod version;
