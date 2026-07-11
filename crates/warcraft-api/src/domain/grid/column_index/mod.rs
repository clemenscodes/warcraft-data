//! [`ColumnIndex`]: the column (0..=3) of a command-card grid slot.

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

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for ColumnIndex {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for ColumnIndex {}
