//! [`RegenRate`]: a per-second regeneration rate on a milli scale (hit points or
//! mana regenerated per second).

/// A per-second regeneration rate, stored as thousandths (milli) per second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RegenRate {
    milli_per_second: u32,
}

impl RegenRate {
    /// From thousandths per second (e.g. `500` = `0.5`/s).
    pub const fn from_milli(milli_per_second: u32) -> Self {
        Self { milli_per_second }
    }

    /// The rate in thousandths per second.
    pub const fn milli(&self) -> u32 {
        self.milli_per_second
    }

    /// The rate as a floating-point value per second (for display).
    pub fn as_f32(&self) -> f32 {
        self.milli_per_second as f32 / 1000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_milli_round_trips() {
        assert_eq!(RegenRate::from_milli(250).milli(), 250);
    }

    #[test]
    fn as_f32_divides_by_a_thousand() {
        assert_eq!(RegenRate::from_milli(250).as_f32(), 0.25);
    }
}
