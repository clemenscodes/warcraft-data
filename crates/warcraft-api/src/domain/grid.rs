//! Command-card grid coordinates. A unit's abilities/build options occupy a
//! fixed 4-column × 3-row button grid; these value objects address a slot in
//! it. This is a layout concern, entirely independent of the object model.

use std::fmt;
use std::str::FromStr;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColumnIndex {
    #[default]
    Zero,
    One,
    Two,
    Three,
}

impl From<ColumnIndex> for u8 {
    fn from(index: ColumnIndex) -> Self {
        match index {
            ColumnIndex::Zero => 0,
            ColumnIndex::One => 1,
            ColumnIndex::Two => 2,
            ColumnIndex::Three => 3,
        }
    }
}

impl TryFrom<u8> for ColumnIndex {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            _ => Err(()),
        }
    }
}

impl From<ColumnIndex> for usize {
    fn from(index: ColumnIndex) -> Self {
        let byte = u8::from(index);
        usize::from(byte)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RowIndex {
    #[default]
    Zero,
    One,
    Two,
}

impl From<RowIndex> for u8 {
    fn from(index: RowIndex) -> Self {
        match index {
            RowIndex::Zero => 0,
            RowIndex::One => 1,
            RowIndex::Two => 2,
        }
    }
}

impl TryFrom<u8> for RowIndex {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            _ => Err(()),
        }
    }
}

impl From<RowIndex> for usize {
    fn from(index: RowIndex) -> Self {
        let byte = u8::from(index);
        usize::from(byte)
    }
}

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

#[derive(Debug)]
pub struct ParseGridCoordinateError;

impl fmt::Display for ParseGridCoordinateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid grid coordinate")
    }
}

impl std::error::Error for ParseGridCoordinateError {}

impl FromStr for GridCoordinate {
    type Err = ParseGridCoordinateError;

    fn from_str(text: &str) -> Result<Self, ParseGridCoordinateError> {
        Self::try_from(text).map_err(|()| ParseGridCoordinateError)
    }
}

// DDD roles: grid coordinates are immutable, equality-by-value → Value Objects.
impl ddd::Layered for ColumnIndex {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for ColumnIndex {}

impl ddd::Layered for RowIndex {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for RowIndex {}

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
