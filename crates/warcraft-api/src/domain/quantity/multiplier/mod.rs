//! [`Multiplier`]: a unitless scaling factor on a milli scale (e.g. a damage
//! effectiveness or an attribute bonus per point).

/// A scaling factor, stored as thousandths (milli).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Multiplier {
    milli: u32,
}

impl Multiplier {
    /// From thousandths (e.g. `1500` = `1.5×`).
    pub const fn from_milli(milli: u32) -> Self {
        Self { milli }
    }

    /// The factor in thousandths.
    pub const fn milli(&self) -> u32 {
        self.milli
    }

    /// The factor as a floating-point multiplier (for display).
    pub fn as_f32(&self) -> f32 {
        self.milli as f32 / 1000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_milli_round_trips() {
        assert_eq!(Multiplier::from_milli(2000).milli(), 2000);
    }

    #[test]
    fn as_f32_divides_by_a_thousand() {
        assert_eq!(Multiplier::from_milli(500).as_f32(), 0.5);
    }
}
