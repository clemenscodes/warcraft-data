//! [`PlayerGameResult`]: how a player's game ended (victory / defeat / tie).

use serde::{Deserialize, Serialize};

use warcraft_primitives::Byte;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum PlayerGameResult {
    Victory = 0,
    Defeat = 1,
    Tie = 2,
    Neutral = 3,
}

impl From<Byte> for PlayerGameResult {
    fn from(value: Byte) -> Self {
        use PlayerGameResult::*;

        match value.get_byte() {
            0 => Victory,
            1 => Defeat,
            2 => Tie,
            3 => Neutral,
            _ => Neutral,
        }
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for PlayerGameResult {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for PlayerGameResult {}
