//! [`DefenseType`]: a unit's armor class (`defenseType` column).

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefenseType {
    Light,
    Medium,
    Heavy,
    Fortified,
    Normal,
    Hero,
    Divine,
    #[default]
    Unarmored,
}

impl DefenseType {
    pub fn parse(raw: &str) -> DefenseType {
        let normalized = raw.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "small" | "light" => DefenseType::Light,
            "medium" => DefenseType::Medium,
            "large" | "heavy" => DefenseType::Heavy,
            "fort" | "fortified" => DefenseType::Fortified,
            "normal" => DefenseType::Normal,
            "hero" => DefenseType::Hero,
            "divine" => DefenseType::Divine,
            _ => DefenseType::Unarmored,
        }
    }

    pub const fn all() -> [DefenseType; 8] {
        [
            DefenseType::Light,
            DefenseType::Medium,
            DefenseType::Heavy,
            DefenseType::Fortified,
            DefenseType::Normal,
            DefenseType::Hero,
            DefenseType::Divine,
            DefenseType::Unarmored,
        ]
    }
}

impl std::fmt::Display for DefenseType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            DefenseType::Light => "Light",
            DefenseType::Medium => "Medium",
            DefenseType::Heavy => "Heavy",
            DefenseType::Fortified => "Fortified",
            DefenseType::Normal => "Normal",
            DefenseType::Hero => "Hero",
            DefenseType::Divine => "Divine",
            DefenseType::Unarmored => "Unarmored",
        };
        formatter.write_str(label)
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for DefenseType {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for DefenseType {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defense_type_parse_aliases() {
        assert_eq!(DefenseType::parse("small"), DefenseType::Light);
        assert_eq!(DefenseType::parse("large"), DefenseType::Heavy);
        assert_eq!(DefenseType::parse("fort"), DefenseType::Fortified);
        assert_eq!(DefenseType::parse("divine"), DefenseType::Divine);
    }

    #[test]
    fn defense_type_all_has_eight_entries() {
        assert_eq!(DefenseType::all().len(), 8);
    }

    #[test]
    fn defense_type_all_contains_every_variant() {
        let all = DefenseType::all();
        assert!(all.contains(&DefenseType::Light));
        assert!(all.contains(&DefenseType::Unarmored));
        assert!(all.contains(&DefenseType::Divine));
    }
}
