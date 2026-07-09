mod ability_tables;
mod balance_overlay_regression;
pub mod catalog;
mod db;
pub mod keybind;
mod keybind_mirrors;
pub mod keycode;
pub mod meta;
pub mod object;
mod object_lookup;
pub mod player;
pub mod primitives;
pub mod race_labels;
mod system_hotkeys_category;
mod test;
mod unit_catalog;
mod unit_kind;
mod unit_mode;
mod variant_groups;
pub mod version;

pub use ability_tables::{
    DEVOUR_ABILITY_ID, HIDDEN_UNIT_ABILITIES, HiddenUnitAbility, PINNED_ROOT_ABILITY_IDS,
    ROOTED_ONLY_ABILITY_CODES, ROOTED_ONLY_ABILITY_IDS,
};
pub use catalog::{BuildingTraits, CommandCatalog};
pub use db::{
    TIERED_UNIT_GROUPS, UNIT_UPGRADE_SWAPS, WARCRAFT_DATABASE, WARCRAFT_GAMEPLAY_CONSTANTS,
    WARCRAFT_SYSTEM_KEYBINDS,
};
pub use keybind_mirrors::{
    BUILD_COMMAND_MIRRORS, BuildCommandMirror, MORPH_ABILITY_MIRRORS, MorphAbilityMirror,
};
pub use object_lookup::ObjectLookup;
pub use system_hotkeys_category::SystemHotkeysCategory;
pub use unit_catalog::{CatalogEntry, CatalogVisibility, SearchField, UnitCatalog};
pub use unit_kind::UnitKindHelpers;
pub use unit_mode::UnitMode;
pub use variant_groups::{VariantGroup, VariantUnits};

pub use keybind::{ContextSet, SystemKeybind, SystemKeybindClass, SystemKeybindModifier};
pub use keycode::KeyCode;
pub use meta::{
    AbilityMeta, AgilityBonuses, AttackType, AttributeBase, AttributeGrowth, CommandMeta,
    DamageEffectiveness, DamageMatrix, DefenseType, GameplayConstants, HeroAttributes,
    IntelligenceBonuses, ItemMeta, ManaPool, PrimaryAttribute, RegenType, StrengthBonuses,
    UnitAttack, UnitCombat, UnitFlags, UnitMeta, UnitProduction, UpgradeMeta, WeaponType,
};
pub use object::{
    ColumnIndex, GridCoordinate, ItemClass, ObjectMap, ParseGridCoordinateError, Race, RowIndex,
    UnitKind, UnitUpgradeSwap, WarcraftDatabase, WarcraftObject, WarcraftObjectId,
    WarcraftObjectKind, WarcraftObjectMeta, WarcraftObjectText,
};
pub use player::{
    AiDifficultyPreference, CampaignMatchType, CustomMatchType, MatchType, MeleeMatchType,
    PlayerColor, PlayerGameResult, PlayerRace, PlayerSlotState, PlayerType, RacePreference, Team,
    TeamPlayer, Teams,
};
pub use primitives::{Boolean, Byte, ByteString, Bytes, Float, Identifier, Integer, Time};
pub use race_labels::{RaceLabels, SUPPORTED_RACES};
pub use version::{SUPPORTED_VERSION, SUPPORTED_VERSION_STRING, WarcraftVersion};
