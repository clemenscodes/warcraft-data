//! [`RacePreference`]: the race a player's slot is set to prefer.

use serde::{Deserialize, Serialize};

use warcraft_primitives::Byte;

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum RacePreference {
    Human = 0x01,
    Orc = 0x02,
    Nightelf = 0x04,
    Undead = 0x08,
    Demon = 0x10,
    #[default]
    Random = 0x20,
    UserSelectable = 0x40,
}

impl From<Byte> for RacePreference {
    fn from(value: Byte) -> Self {
        use RacePreference::*;
        // UserSelectable is masked into the value in memory, so just add its value 0x40
        match value.get_byte() {
            0x41 | 0x01 => Human,
            0x42 | 0x02 => Orc,
            0x44 | 0x04 => Nightelf,
            0x48 | 0x08 => Undead,
            0x50 | 0x10 => Demon,
            0x60 | 0x20 => Random,
            _ => Self::default(),
        }
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for RacePreference {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for RacePreference {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn race_preference_from_byte_handles_user_selectable_mask() {
        // 0x41 = Human | UserSelectable
        assert!(matches!(
            RacePreference::from(Byte::from(0x41u8)),
            RacePreference::Human
        ));
        assert!(matches!(
            RacePreference::from(Byte::from(0x01u8)),
            RacePreference::Human
        ));
    }
}
