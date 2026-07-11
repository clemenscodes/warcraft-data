//! [`Race`]: the Warcraft III race an object belongs to.

use std::fmt;

/// The Warcraft III race an object belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Race {
    Human,
    Nightelf,
    Orc,
    Undead,
    Neutral,
}

impl TryFrom<&str> for Race {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lowercased = value.to_lowercase();
        let normalized = lowercased.trim();
        match normalized {
            "human" => Ok(Self::Human),
            "orc" => Ok(Self::Orc),
            "nightelf" | "night elf" => Ok(Self::Nightelf),
            "undead" => Ok(Self::Undead),
            "neutral" => Ok(Self::Neutral),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Race {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Human => "Human",
            Self::Orc => "Orc",
            Self::Nightelf => "Night Elf",
            Self::Undead => "Undead",
            Self::Neutral => "Neutral",
        };
        formatter.write_str(name)
    }
}

// DDD role: a race is an immutable Value Object.
impl ddd::Layered for Race {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for Race {}
