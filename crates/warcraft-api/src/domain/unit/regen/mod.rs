//! [`RegenType`]: when a unit regenerates its hit points.

// Mirrors the `regenType` column in `unitbalance.slk`. Controls when HP
// regeneration is active; the rate (`hit_points_regen`) is the per-second
// value WHILE active, with no day/night multiplier on top of it.
//
//   Always — regenerates anywhere, anytime (Human, Orc, neutral creeps).
//   Night  — regenerates only between dusk and dawn (Night Elf).
//   Blight — regenerates only while standing on blight (Undead).
//   None   — does not regenerate HP at all (some neutral structures /
//            mechanical creeps).
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegenType {
    #[default]
    Always,
    Night,
    Blight,
    None,
}

impl RegenType {
    pub fn parse(raw_value: &str) -> Self {
        match raw_value.trim().to_ascii_lowercase().as_str() {
            "always" => Self::Always,
            "night" => Self::Night,
            "blight" => Self::Blight,
            "none" | "" | "-" | "_" => Self::None,
            _ => Self::Always,
        }
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for RegenType {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for RegenType {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regen_type_parse_known_values() {
        assert_eq!(RegenType::parse("always"), RegenType::Always);
        assert_eq!(RegenType::parse("night"), RegenType::Night);
        assert_eq!(RegenType::parse("blight"), RegenType::Blight);
        assert_eq!(RegenType::parse("none"), RegenType::None);
    }

    #[test]
    fn regen_type_parse_empty_and_dash_are_none() {
        assert_eq!(RegenType::parse(""), RegenType::None);
        assert_eq!(RegenType::parse("-"), RegenType::None);
        assert_eq!(RegenType::parse("_"), RegenType::None);
    }
}
