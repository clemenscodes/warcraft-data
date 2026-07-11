//! [`GridCoordinate`]: a column/row slot in the command-card button grid.

use std::fmt;
use std::str::FromStr;

use crate::domain::grid::column_index::ColumnIndex;
use crate::domain::grid::parse_error::ParseGridCoordinateError;
use crate::domain::grid::row_index::RowIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoordinate {
    column: ColumnIndex,
    row: RowIndex,
}

impl GridCoordinate {
    pub const fn new(column: ColumnIndex, row: RowIndex) -> Self {
        Self { column, row }
    }

    pub fn column(self) -> ColumnIndex {
        self.column
    }

    pub fn row(self) -> RowIndex {
        self.row
    }
}

impl Default for GridCoordinate {
    fn default() -> Self {
        Self {
            column: ColumnIndex::Zero,
            row: RowIndex::Zero,
        }
    }
}

impl TryFrom<&str> for GridCoordinate {
    type Error = ();

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let mut parts = text.splitn(2, ',');
        let column = parts
            .next()
            .ok_or(())?
            .trim()
            .parse::<u8>()
            .map_err(|_| ())?;
        let row = parts
            .next()
            .ok_or(())?
            .trim()
            .parse::<u8>()
            .map_err(|_| ())?;
        let column = ColumnIndex::try_from(column)?;
        let row = RowIndex::try_from(row)?;
        Ok(GridCoordinate { column, row })
    }
}

impl fmt::Display for GridCoordinate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let column = u8::from(self.column);
        let row = u8::from(self.row);
        write!(formatter, "{column},{row}")
    }
}

impl FromStr for GridCoordinate {
    type Err = ParseGridCoordinateError;

    fn from_str(text: &str) -> Result<Self, ParseGridCoordinateError> {
        Self::try_from(text).map_err(|()| ParseGridCoordinateError)
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for GridCoordinate {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for GridCoordinate {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_coordinate_stores_column_and_row() {
        let coordinate = GridCoordinate::new(ColumnIndex::Three, RowIndex::One);
        assert_eq!(coordinate.column(), ColumnIndex::Three);
        assert_eq!(coordinate.row(), RowIndex::One);
    }

    #[test]
    fn grid_coordinate_parses_from_column_row_text() {
        let coordinate: GridCoordinate = "3,1".parse().expect("valid coordinate");
        assert_eq!(coordinate.column(), ColumnIndex::Three);
        assert_eq!(coordinate.row(), RowIndex::One);
    }
}
