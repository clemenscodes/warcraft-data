//! Infrastructure layer: the backing store. `pub(crate)` — never public API;
//! the application layer (`WarcraftApi`) is the only way in.

pub(crate) mod database;
