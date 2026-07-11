//! [`Chance`]: a probability on a permille scale (0..=1000 = 0%..=100%).

/// A probability, stored as permille (thousandths of 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Chance {
    permille: u16,
}

impl Chance {
    /// From permille (e.g. `150` = `15%`).
    pub const fn from_permille(permille: u16) -> Self {
        Self { permille }
    }

    /// The probability in permille (0..=1000).
    pub const fn permille(&self) -> u16 {
        self.permille
    }

    /// The probability as a fraction in 0.0..=1.0 (for display).
    pub fn as_fraction(&self) -> f32 {
        self.permille as f32 / 1000.0
    }

    /// Whether the chance is non-zero.
    pub const fn is_some(&self) -> bool {
        self.permille > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_permille_round_trips() {
        assert_eq!(Chance::from_permille(150).permille(), 150);
    }

    #[test]
    fn as_fraction_divides_by_a_thousand() {
        assert_eq!(Chance::from_permille(150).as_fraction(), 0.15);
    }

    #[test]
    fn is_some_reflects_non_zero() {
        assert!(Chance::from_permille(1).is_some());
        assert!(!Chance::from_permille(0).is_some());
    }
}
