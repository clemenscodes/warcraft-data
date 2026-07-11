//! [`Armor`]: a unit's armor value on a milli scale. Signed, because armor can
//! be negative.

/// A unit's armor, stored as thousandths (milli) of an armor point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Armor {
    milli: i32,
}

impl Armor {
    /// From thousandths of an armor point (e.g. `1500` = `1.5` armor).
    pub const fn from_milli(milli: i32) -> Self {
        Self { milli }
    }

    /// The value in thousandths of an armor point.
    pub const fn milli(&self) -> i32 {
        self.milli
    }

    /// The value as a floating-point armor value (for display).
    pub fn as_f32(&self) -> f32 {
        self.milli as f32 / 1000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_milli_round_trips() {
        assert_eq!(Armor::from_milli(1500).milli(), 1500);
        assert_eq!(Armor::from_milli(-2000).milli(), -2000);
    }

    #[test]
    fn as_f32_divides_by_a_thousand() {
        assert_eq!(Armor::from_milli(1500).as_f32(), 1.5);
    }

    #[test]
    fn equal_values_are_equal() {
        assert_eq!(Armor::from_milli(500), Armor::from_milli(500));
    }
}
