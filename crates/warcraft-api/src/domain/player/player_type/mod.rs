//! [`PlayerType`]: what occupies a player slot (human / computer / observer …).

use serde::{Deserialize, Serialize};

use warcraft_primitives::Byte;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum PlayerType {
    Empty = 0,
    Player = 1,
    Computer = 2,
    Neutral = 3,
    Observer = 4,
    None = 5,
    Other = 6,
}

impl From<Byte> for PlayerType {
    fn from(value: Byte) -> Self {
        use PlayerType::*;

        match value.get_byte() {
            0 => Empty,
            1 => Player,
            2 => Computer,
            3 => Neutral,
            4 => Observer,
            5 => None,
            6 => Other,
            _ => Empty,
        }
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for PlayerType {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for PlayerType {}
