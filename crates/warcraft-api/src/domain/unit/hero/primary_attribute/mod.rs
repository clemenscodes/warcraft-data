//! [`PrimaryAttribute`]: which attribute a hero's damage scales with.

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryAttribute {
    #[default]
    Strength,
    Agility,
    Intelligence,
}

impl PrimaryAttribute {
    pub fn parse(raw: &str) -> Option<PrimaryAttribute> {
        let normalized = raw.trim().to_ascii_uppercase();
        match normalized.as_str() {
            "STR" => Some(PrimaryAttribute::Strength),
            "AGI" => Some(PrimaryAttribute::Agility),
            "INT" => Some(PrimaryAttribute::Intelligence),
            _ => None,
        }
    }
}

impl std::fmt::Display for PrimaryAttribute {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            PrimaryAttribute::Strength => "Strength",
            PrimaryAttribute::Agility => "Agility",
            PrimaryAttribute::Intelligence => "Intelligence",
        };
        formatter.write_str(label)
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for PrimaryAttribute {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for PrimaryAttribute {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_attribute_parse_case_insensitive() {
        assert_eq!(
            PrimaryAttribute::parse("str"),
            Some(PrimaryAttribute::Strength)
        );
        assert_eq!(
            PrimaryAttribute::parse("AGI"),
            Some(PrimaryAttribute::Agility)
        );
        assert_eq!(
            PrimaryAttribute::parse("int"),
            Some(PrimaryAttribute::Intelligence)
        );
    }

    #[test]
    fn primary_attribute_parse_unknown_is_none() {
        assert_eq!(PrimaryAttribute::parse("xyz"), None);
    }

    #[test]
    fn primary_attribute_display_is_full_name() {
        assert_eq!(PrimaryAttribute::Strength.to_string(), "Strength");
        assert_eq!(PrimaryAttribute::Agility.to_string(), "Agility");
        assert_eq!(PrimaryAttribute::Intelligence.to_string(), "Intelligence");
    }
}
