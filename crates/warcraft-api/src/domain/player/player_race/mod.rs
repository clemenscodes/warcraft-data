//! [`PlayerRace`]: the actual race a player played as.

use serde::{Deserialize, Serialize};

use warcraft_primitives::Byte;

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum PlayerRace {
    #[default]
    Unknown = 0,
    Human = 1,
    Orc = 2,
    Undead = 3,
    NightElf = 4,
    Demon = 5,
    Last = 6,
    Other = 7,
    Creep = 8,
    Commoner = 9,
    Critter = 10,
    Naga = 11,
}

impl From<Byte> for PlayerRace {
    fn from(value: Byte) -> Self {
        use PlayerRace::*;

        match value.get_byte() {
            0 => Unknown,
            1 => Human,
            2 => Orc,
            3 => Undead,
            4 => NightElf,
            5 => Demon,
            6 => Last,
            7 => Other,
            8 => Creep,
            9 => Commoner,
            10 => Critter,
            11 => Naga,
            _ => Unknown,
        }
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for PlayerRace {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for PlayerRace {}
