//! [`StatGrowth`]: a hero attribute's per-level gain on a milli scale.

/// A per-level attribute gain, stored as thousandths (milli) per level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StatGrowth {
    milli_per_level: u32,
}

impl StatGrowth {
    /// From thousandths per level (e.g. `2700` = `2.7`/level).
    pub const fn from_milli(milli_per_level: u32) -> Self {
        Self { milli_per_level }
    }

    /// The gain in thousandths per level.
    pub const fn milli(&self) -> u32 {
        self.milli_per_level
    }

    /// The gain as a floating-point value per level (for display).
    pub fn as_f32(&self) -> f32 {
        self.milli_per_level as f32 / 1000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_milli_round_trips() {
        assert_eq!(StatGrowth::from_milli(2700).milli(), 2700);
    }

    #[test]
    fn as_f32_divides_by_a_thousand() {
        assert_eq!(StatGrowth::from_milli(2700).as_f32(), 2.7);
    }
}
