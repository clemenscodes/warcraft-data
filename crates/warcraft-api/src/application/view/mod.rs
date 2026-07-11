//! Flat read-model projections returned by the domain sub-APIs. A `*View` holds
//! no database handle: it is the result of a query and exposes only its own
//! entity's data. Navigation between entities is the caller's job, done through
//! the `*Api` services — a view never queries.

pub(crate) mod ability;
pub(crate) mod unit;
