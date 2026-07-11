mod ability_tables;
mod application;
pub mod catalog;
mod db;
mod domain;
mod infrastructure;
pub mod keybind;
mod keybind_mirrors;
pub mod keycode;
pub mod meta;
mod system_hotkeys_category;
mod unit_catalog;
mod unit_kind;
mod variant_groups;

pub use ability_tables::{
    DEVOUR_ABILITY_ID, HIDDEN_UNIT_ABILITIES, HiddenUnitAbility, PINNED_ROOT_ABILITY_IDS,
    ROOTED_ONLY_ABILITY_CODES, ROOTED_ONLY_ABILITY_IDS,
};
pub use application::api::WarcraftApi;
pub use catalog::{BuildingTraits, CommandCatalog};
pub use db::{
    TIERED_UNIT_GROUPS, UNIT_UPGRADE_SWAPS, WARCRAFT_GAMEPLAY_CONSTANTS, WARCRAFT_SYSTEM_KEYBINDS,
};
pub use keybind_mirrors::{
    BUILD_COMMAND_MIRRORS, BuildCommandMirror, MORPH_ABILITY_MIRRORS, MorphAbilityMirror,
};
pub use system_hotkeys_category::SystemHotkeysCategory;
pub use unit_catalog::{CatalogEntry, CatalogVisibility, SearchField, UnitCatalog};
pub use unit_kind::UnitKindHelpers;
pub use domain::unit::UnitMode;
pub use variant_groups::{VariantGroup, VariantUnits};

pub use keybind::{ContextSet, SystemKeybind, SystemKeybindClass, SystemKeybindModifier};
pub use keycode::KeyCode;
pub use meta::{
    AbilityMeta, AgilityBonuses, DamageEffectiveness, DamageMatrix, GameplayConstants,
    IntelligenceBonuses, StrengthBonuses, UnitAttack, UnitCombat, UnitMeta,
};
pub use domain::combat::{AttackType, DefenseType, WeaponType};
pub use domain::command::CommandMeta;
pub use domain::hero::{
    AttributeBase, AttributeGrowth, HeroAttributes, ManaPool, PrimaryAttribute,
};
pub use domain::grid::{ColumnIndex, GridCoordinate, ParseGridCoordinateError, RowIndex};
pub use domain::identity::WarcraftObjectId;
pub use domain::item::{ItemClass, ItemMeta};
pub use domain::upgrade::{UnitUpgradeSwap, UpgradeMeta};
pub use domain::object::{
    WarcraftObject, WarcraftObjectKind, WarcraftObjectMeta, WarcraftObjectText,
};
pub use domain::unit::{RegenType, UnitFlags, UnitKind, UnitProduction};
pub(crate) use infrastructure::database::WarcraftDatabase;
pub use domain::player::{
    AiDifficultyPreference, CampaignMatchType, CustomMatchType, MatchType, MeleeMatchType,
    PlayerColor, PlayerGameResult, PlayerRace, PlayerSlotState, PlayerType, RacePreference, Team,
    TeamPlayer, Teams,
};
pub use domain::race::{AllRaces, Race};
pub use domain::version::{SUPPORTED_VERSION, SUPPORTED_VERSION_STRING, WarcraftVersion};
