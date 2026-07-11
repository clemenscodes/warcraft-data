//! [`WeaponType`]: how a unit's weapon behaves (`weapTp1` column). Distinct from
//! the damage `AttackType`; the two artillery variants grant the Attack Ground
//! command in-game.

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

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for WeaponType {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for WeaponType {}

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
}
