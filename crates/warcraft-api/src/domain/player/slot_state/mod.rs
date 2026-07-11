//! [`PlayerSlotState`]: whether a player slot is empty, playing, or left.

use serde::{Deserialize, Serialize};

use warcraft_primitives::Byte;

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum PlayerSlotState {
    #[default]
    Empty = 0,
    Playing = 1,
    Left = 2,
}

impl From<Byte> for PlayerSlotState {
    fn from(value: Byte) -> Self {
        use PlayerSlotState::*;

        match value.get_byte() {
            0 => Empty,
            1 => Playing,
            2 => Left,
            _ => Self::default(),
        }
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for PlayerSlotState {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for PlayerSlotState {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_slot_state_default_is_empty() {
        assert_eq!(PlayerSlotState::default(), PlayerSlotState::Empty);
    }

    #[test]
    fn player_slot_state_from_byte_round_trips() {
        assert!(matches!(
            PlayerSlotState::from(Byte::from(1u8)),
            PlayerSlotState::Playing
        ));
        assert!(matches!(
            PlayerSlotState::from(Byte::from(2u8)),
            PlayerSlotState::Left
        ));
    }
}
