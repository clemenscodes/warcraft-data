mod application;
mod domain;
mod infrastructure;

pub use application::ability::AbilityApi;
pub use application::api::WarcraftApi;
pub use application::command::CommandApi;
pub use application::unit::UnitApi;
pub use application::unit::listing::query::{Scope, UnitQuery};
pub use application::unit::listing::search_field::SearchField;
pub use application::unit::listing::visibility::CatalogVisibility;
pub use application::unit::{
    UnitCategoryEntry, UnitCategoryListing, UnitCategoryRequest, UnitListing, UnitListingEntry,
    UnitListingRequest,
};
pub use application::view::ability::AbilityView;
pub use application::view::command::CommandView;
pub use application::view::unit::UnitView;
pub use domain::ability::AbilityMeta;
pub use domain::balance::{
    AgilityBonuses, DamageEffectiveness, DamageMatrix, GameplayConstants, IntelligenceBonuses,
    StrengthBonuses,
};
pub use domain::combat::{AttackType, DefenseType, WeaponType};
pub use domain::command::{CommandLabel, CommandMeta};
pub use domain::grid::{ColumnIndex, GridCoordinate, ParseGridCoordinateError, RowIndex};
pub use domain::identity::WarcraftObjectId;
pub use domain::item::{ItemClass, ItemMeta};
pub use domain::keybind::ability_tables::{
    DEVOUR_ABILITY_ID, HIDDEN_UNIT_ABILITIES, HiddenUnitAbility, PINNED_ROOT_ABILITY_IDS,
    ROOTED_ONLY_ABILITY_CODES, ROOTED_ONLY_ABILITY_IDS,
};
pub use domain::keybind::category::SystemHotkeysCategory;
pub use domain::keybind::keycode::{
    Digit, FunctionKey, KeyCode, KeyCodeOutOfRange, Letter, MouseButton, NotALetter, NumpadKey,
    Punctuation,
};
pub use domain::keybind::mirrors::{
    BUILD_COMMAND_MIRRORS, BuildCommandMirror, MORPH_ABILITY_MIRRORS, MorphAbilityMirror,
};
pub use domain::keybind::system_keybind::{
    ContextSet, SystemKeybind, SystemKeybindClass, SystemKeybindModifier,
};
pub use domain::object::{
    Description, Tip, WarcraftColorCodes, WarcraftObject, WarcraftObjectKind, WarcraftObjectMeta,
    WarcraftObjectText,
};
pub use domain::player::{
    AiDifficultyPreference, CampaignMatchType, CustomMatchType, MatchType, MeleeMatchType,
    PlayerColor, PlayerGameResult, PlayerRace, PlayerSlotState, PlayerType, RacePreference, Team,
    TeamPlayer, Teams,
};
pub use domain::quantity::{Armor, Chance, Cooldown, Multiplier, RegenRate, StatGrowth};
pub use domain::race::{AllRaces, Race};
pub use domain::statistics::{
    ArmorFigure, AttackRange, AttackSpeed, AttackStatistics, AttributeStatistic, DamagePerSecond,
    DamageRange, EffectiveHitPoints, Evasion, Gain, HeroStatistics, HitPoints, HitPointsRegen,
    Mana, ManaRegen, Matchup, MatchupStrength, UnitStatistics,
};
pub use domain::unit::hero::{
    AttributeBase, AttributeGrowth, HeroAttributes, ManaPool, PrimaryAttribute,
};
pub use domain::unit::{
    RegenType, UnitAttack, UnitCombat, UnitFlags, UnitKind, UnitMeta, UnitMode, UnitProduction,
};
pub use domain::upgrade::{UnitUpgradeSwap, UpgradeMeta};
pub use domain::version::{SUPPORTED_VERSION, SUPPORTED_VERSION_STRING, WarcraftVersion};
pub use infrastructure::database::generated::{
    TIERED_UNIT_GROUPS, UNIT_UPGRADE_SWAPS, WARCRAFT_GAMEPLAY_CONSTANTS, WARCRAFT_SYSTEM_KEYBINDS,
};
// Infrastructure type. Kept crate-internal (never public API), but visible at the
// crate root so the generated `database/generated.rs` (which imports `crate::*`)
// resolves it.
pub(crate) use infrastructure::database::WarcraftDatabase;
