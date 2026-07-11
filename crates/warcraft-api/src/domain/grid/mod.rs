//! Command-card grid coordinates. A unit's abilities/build options occupy a
//! fixed 4-column × 3-row button grid; these value objects address a slot in it.
//! This is a layout concern, entirely independent of the object model.

pub(crate) mod column_index;
pub(crate) mod coordinate;
pub(crate) mod parse_error;
pub(crate) mod row_index;

pub use column_index::ColumnIndex;
pub use coordinate::GridCoordinate;
pub use parse_error::ParseGridCoordinateError;
pub use row_index::RowIndex;
