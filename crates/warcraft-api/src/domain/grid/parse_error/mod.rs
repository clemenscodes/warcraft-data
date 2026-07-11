//! [`ParseGridCoordinateError`]: returned when a string is not a valid
//! `column,row` grid coordinate.

use std::fmt;

#[derive(Debug)]
pub struct ParseGridCoordinateError;

impl fmt::Display for ParseGridCoordinateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid grid coordinate")
    }
}

impl std::error::Error for ParseGridCoordinateError {}
