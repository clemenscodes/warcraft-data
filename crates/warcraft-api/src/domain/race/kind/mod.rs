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

impl Race {
    /// The lowercase, spaceless slug for this race — the serialize inverse of
    /// [`Race::try_from`]: `Race::try_from(race.slug())` round-trips back to
    /// `race` for every race.
    pub fn slug(self) -> &'static str {
        match self {
            Self::Human => "human",
            Self::Orc => "orc",
            Self::Nightelf => "nightelf",
            Self::Undead => "undead",
            Self::Neutral => "neutral",
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AllRaces;

    #[test]
    fn slug_round_trips_through_try_from_for_every_race() {
        for race in AllRaces::ALL.iter() {
            let slug = race.slug();
            assert_eq!(
                Race::try_from(slug),
                Ok(race),
                "slug round trip failed for {race}"
            );
        }
    }
}
