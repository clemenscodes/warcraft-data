//! Combat taxonomy: the attack/weapon/defense type enums. Shared vocabulary —
//! consumed both by unit combat stats and by the balance damage matrix.

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

// Mirrors the `weapTp1` column in `unitweapons.slk`. This is distinct from
// `AttackType` (the damage type): the weapon type controls attack behavior, and
// the two artillery variants are the ones that grant a unit the Attack Ground
// command in-game.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    Normal,
    Instant,
    Artillery,
    ArtilleryLine,
    Missile,
    MissileSplash,
    MissileBounce,
    MissileLine,
    None,
    #[default]
    Unknown,
}

impl WeaponType {
    pub fn parse(raw: &str) -> WeaponType {
        let normalized = raw.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "normal" => WeaponType::Normal,
            "instant" => WeaponType::Instant,
            "artillery" => WeaponType::Artillery,
            "aline" => WeaponType::ArtilleryLine,
            "missile" => WeaponType::Missile,
            "msplash" => WeaponType::MissileSplash,
            "mbounce" => WeaponType::MissileBounce,
            "mline" => WeaponType::MissileLine,
            "none" => WeaponType::None,
            _ => WeaponType::Unknown,
        }
    }

    pub fn targets_ground(&self) -> bool {
        matches!(self, WeaponType::Artillery | WeaponType::ArtilleryLine)
    }
}

impl std::fmt::Display for WeaponType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            WeaponType::Normal => "Normal",
            WeaponType::Instant => "Instant",
            WeaponType::Artillery => "Artillery (Targets Ground)",
            WeaponType::ArtilleryLine => "Artillery (Line)",
            WeaponType::Missile => "Missile",
            WeaponType::MissileSplash => "Missile (Splash)",
            WeaponType::MissileBounce => "Missile (Bounce)",
            WeaponType::MissileLine => "Missile (Line)",
            WeaponType::None => "None",
            WeaponType::Unknown => "Unknown",
        };
        formatter.write_str(label)
    }
}

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

// DDD roles: combat type enums are immutable, equality-by-value → Value Objects.
impl ddd::Layered for AttackType {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for AttackType {}

impl ddd::Layered for WeaponType {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for WeaponType {}

impl ddd::Layered for DefenseType {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for DefenseType {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weapon_type_parse_artillery_variants() {
        assert_eq!(WeaponType::parse("artillery"), WeaponType::Artillery);
        assert_eq!(WeaponType::parse("aline"), WeaponType::ArtilleryLine);
        assert_eq!(WeaponType::parse("MISSILE"), WeaponType::Missile);
        assert_eq!(WeaponType::parse("nonsense"), WeaponType::Unknown);
    }

    #[test]
    fn weapon_type_targets_ground_only_for_artillery() {
        assert!(WeaponType::Artillery.targets_ground());
        assert!(WeaponType::ArtilleryLine.targets_ground());
        assert!(!WeaponType::Missile.targets_ground());
        assert!(!WeaponType::Normal.targets_ground());
    }

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
