//! [`RowIndex`]: the row (0..=2) of a command-card grid slot.

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

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for RowIndex {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for RowIndex {}
