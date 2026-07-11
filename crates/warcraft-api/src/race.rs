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

// The five races, each as its own distinct type. Because the types differ, no
// value of one can ever stand in for another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Human;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Orc;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NightElf;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Undead;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Neutral;

impl From<Human> for Race {
    fn from(_: Human) -> Self {
        Self::Human
    }
}
impl From<Orc> for Race {
    fn from(_: Orc) -> Self {
        Self::Orc
    }
}
impl From<NightElf> for Race {
    fn from(_: NightElf) -> Self {
        Self::Nightelf
    }
}
impl From<Undead> for Race {
    fn from(_: Undead) -> Self {
        Self::Undead
    }
}
impl From<Neutral> for Race {
    fn from(_: Neutral) -> Self {
        Self::Neutral
    }
}

/// The set of all races. Its **only** inhabitant is the five distinct races,
/// each exactly once: every position is a different type, so a duplicate
/// (`Human` twice), an omission, or a sixth entry does not type-check. There is
/// nothing to get wrong — the fields are private, the one value is [`AllRaces::ALL`],
/// and a consumer only ever iterates it with [`AllRaces::iter`], never naming a
/// single race.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AllRaces(Human, Orc, NightElf, Undead, Neutral);

impl AllRaces {
    pub const ALL: Self = Self(Human, Orc, NightElf, Undead, Neutral);

    /// The five races, in canonical (tab) order, as plain `Race` values for a
    /// consumer that just wants to loop.
    pub fn iter(self) -> impl Iterator<Item = Race> {
        let races = [
            Race::from(self.0),
            Race::from(self.1),
            Race::from(self.2),
            Race::from(self.3),
            Race::from(self.4),
        ];
        races.into_iter()
    }
}
