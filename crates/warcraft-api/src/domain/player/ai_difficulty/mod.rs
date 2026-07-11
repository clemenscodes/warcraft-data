//! [`AiDifficultyPreference`]: the difficulty a computer player is set to.

use serde::{Deserialize, Serialize};

use warcraft_primitives::Byte;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum AiDifficultyPreference {
    Newbie = 0,
    Normal = 1,
    Insane = 2,
}

impl From<Byte> for AiDifficultyPreference {
    fn from(value: Byte) -> Self {
        use AiDifficultyPreference::*;

        match value.get_byte() {
            0 => Newbie,
            1 => Normal,
            2 => Insane,
            _ => Newbie,
        }
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for AiDifficultyPreference {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for AiDifficultyPreference {}
