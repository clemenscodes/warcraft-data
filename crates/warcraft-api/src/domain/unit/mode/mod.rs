//! [`UnitMode`]: the game mode a unit list is filtered for (melee / campaign).

/// The game mode a unit list is filtered for: standard melee, or campaign.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnitMode {
    Melee,
    Campaign,
}

impl TryFrom<&str> for UnitMode {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "melee" => Ok(Self::Melee),
            "campaign" => Ok(Self::Campaign),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for UnitMode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Melee => formatter.write_str("melee"),
            Self::Campaign => formatter.write_str("campaign"),
        }
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for UnitMode {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for UnitMode {}
