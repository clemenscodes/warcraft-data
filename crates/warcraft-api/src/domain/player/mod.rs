//! Player domain concept: player/slot setup and match-configuration value
//! objects (as read from a replay's player records).

pub(crate) mod ai_difficulty;
pub(crate) mod color;
pub(crate) mod game_result;
pub(crate) mod match_type;
pub(crate) mod player_race;
pub(crate) mod player_type;
pub(crate) mod race_preference;
pub(crate) mod slot_state;
pub(crate) mod team;

pub use ai_difficulty::AiDifficultyPreference;
pub use color::PlayerColor;
pub use game_result::PlayerGameResult;
pub use match_type::{CampaignMatchType, CustomMatchType, MatchType, MeleeMatchType};
pub use player_race::PlayerRace;
pub use player_type::PlayerType;
pub use race_preference::RacePreference;
pub use slot_state::PlayerSlotState;
pub use team::{Team, TeamPlayer, Teams};
