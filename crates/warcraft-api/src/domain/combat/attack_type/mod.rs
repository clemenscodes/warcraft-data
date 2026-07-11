//! [`AttackType`]: the damage type of a unit's attack (`attackType` column).

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackType {
    Normal,
    Pierce,
    Siege,
    Magic,
    Chaos,
    Hero,
    Spells,
    #[default]
    Unknown,
}

impl AttackType {
    pub fn parse(raw: &str) -> AttackType {
        let normalized = raw.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "normal" => AttackType::Normal,
            "pierce" => AttackType::Pierce,
            "siege" => AttackType::Siege,
            "magic" => AttackType::Magic,
            "chaos" => AttackType::Chaos,
            "hero" => AttackType::Hero,
            "spells" => AttackType::Spells,
            _ => AttackType::Unknown,
        }
    }
}

impl std::fmt::Display for AttackType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            AttackType::Normal => "Normal",
            AttackType::Pierce => "Piercing",
            AttackType::Siege => "Siege",
            AttackType::Magic => "Magic",
            AttackType::Chaos => "Chaos",
            AttackType::Hero => "Hero",
            AttackType::Spells => "Spells",
            AttackType::Unknown => "Unknown",
        };
        formatter.write_str(label)
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for AttackType {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for AttackType {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attack_type_parse_known_values() {
        assert_eq!(AttackType::parse("normal"), AttackType::Normal);
        assert_eq!(AttackType::parse("chaos"), AttackType::Chaos);
        assert_eq!(AttackType::parse("spells"), AttackType::Spells);
    }

    #[test]
    fn attack_type_parse_unknown_falls_back_to_unknown() {
        assert_eq!(AttackType::parse("garbage"), AttackType::Unknown);
    }
}
