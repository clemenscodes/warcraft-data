//! [`TeamPlayer`] and the [`Team`]/[`Teams`] maps: the per-slot player setup
//! grouped by team.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::domain::player::color::PlayerColor;
use crate::domain::player::race_preference::RacePreference;
use crate::domain::player::slot_state::PlayerSlotState;

pub type Team = BTreeMap<u32, TeamPlayer>;
pub type Teams = BTreeMap<u8, Team>;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TeamPlayer {
    name: String,
    race_preference: RacePreference,
    state: PlayerSlotState,
    color: PlayerColor,
}

impl TeamPlayer {
    pub fn new(
        name: String,
        race_preference: RacePreference,
        state: PlayerSlotState,
        color: PlayerColor,
    ) -> Self {
        Self {
            name,
            race_preference,
            state,
            color,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn race_preference(&self) -> RacePreference {
        self.race_preference
    }

    pub fn state(&self) -> PlayerSlotState {
        self.state
    }

    pub fn color(&self) -> PlayerColor {
        self.color
    }
}

// DDD role: player setup value object (equality-by-value). `Team`/`Teams` are
// plain `BTreeMap` aliases and carry no role.
impl ddd::Layered for TeamPlayer {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for TeamPlayer {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn team_player_accessors_return_stored_values() {
        let player = TeamPlayer::new(
            String::from("Alice"),
            RacePreference::Human,
            PlayerSlotState::Playing,
            PlayerColor::Teal,
        );
        assert_eq!(player.name(), "Alice");
        assert!(matches!(player.state(), PlayerSlotState::Playing));
        assert!(matches!(player.color(), PlayerColor::Teal));
    }
}
