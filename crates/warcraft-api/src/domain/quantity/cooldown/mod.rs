//! [`Cooldown`]: a weapon's attack cooldown, stored as milliseconds so it stays
//! an exact, equality-comparable value (the source data is fractional seconds).

/// An attack cooldown, stored in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Cooldown {
    millis: u32,
}

impl Cooldown {
    /// From milliseconds (e.g. `1500` = `1.5` seconds).
    pub const fn from_millis(millis: u32) -> Self {
        Self { millis }
    }

    /// The cooldown in milliseconds.
    pub const fn millis(&self) -> u32 {
        self.millis
    }

    /// The cooldown in seconds (for display).
    pub fn as_secs_f32(&self) -> f32 {
        self.millis as f32 / 1000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_millis_round_trips() {
        assert_eq!(Cooldown::from_millis(1500).millis(), 1500);
    }

    #[test]
    fn as_secs_divides_by_a_thousand() {
        assert_eq!(Cooldown::from_millis(1500).as_secs_f32(), 1.5);
    }
}
