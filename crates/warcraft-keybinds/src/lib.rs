pub use warcraft_api::{SystemKeybindClass, SystemKeybindModifier, WarcraftObjectId};

pub mod cascade;
pub mod collision;
pub mod command;
pub mod custom_keys;
pub mod display;
pub mod editor_history;
pub mod grid;
pub mod identity;
pub mod model;
pub mod statistics;
pub mod system;
pub mod text;
pub mod unit;

pub use cascade::conflict_graph::{CollidingPair, ConflictGraph, ConflictNode};
pub use cascade::planner::{CascadePlan, MoveReason, PlannedMove, UnresolvedMover};
pub use cascade::queue::{AssignmentQueue, AssignmentScope, GroupKind, PositionAssignmentGroup};

pub use collision::cross_unit::{
    AffectedUnitEntry, CrossUnitCollisionReport, CrossUnitPositionGroup, SharedAbilityEntry,
};

pub use collision::summary::CollisionSummary;
pub use collision::unit_report::{UnitCollisionEntry, UnitCollisionReport};
pub use command::move_request::MoveRequest;
pub use custom_keys::{CustomKeys, DEFAULT_CUSTOM_KEYS, HotkeyConflict, ImportOutcome};
pub use display::ability_cell::{AbilityCell, AbilityIconPath};

pub use display::grid_behavior::{
    AlternateFormBehavior, CommandBehavior, GridBehavior, ResearchBehavior,
};

pub use display::inspector_detail::InspectorDetail;
pub use display::rendered_grid::{CommandGridRenderInput, RenderedTile};
pub use display::templates::{BundledTemplate, ResolvedTemplate};

pub use grid::layout::{
    COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, COMMAND_GRID_TILE_COUNT, GridLayout,
};

pub use editor_history::{EditorHistory, EditorSnapshot};

pub use identity::ability_id::AbilityId;
pub use identity::hotkey_target::HotkeyTarget;
pub use identity::hotkey_token::{HotkeyToken, HotkeyTokenIsNotLetter, HotkeyTokenParseError};

pub use identity::keycode::{
    Digit, FunctionKey, KeyCode, KeyCodeOutOfRange, Letter, MouseButton, NotALetter, NumpadKey,
    Punctuation,
};

pub use identity::slot::{CommandCard, GridSlotId};

pub use model::{
    AbilityBinding, AbilityBindingBuilder, AbilityModifier, BindingEntry, ColumnIndex,
    CommandBinding, CommandBindingBuilder, CommandEntry, CustomKeysBuilder, GridCoordinate, Hotkey,
    RowIndex, SystemBinding, WarcraftKeybinding,
};

pub use statistics::{
    Armor, AttackRange, AttackSpeed, AttackStatistics, AttributeStatistic, DamagePerSecond,
    DamageRange, EffectiveHitPoints, Evasion, Gain, HeroStatistics, HitPoints, HitPointsRegen,
    Mana, ManaRegen, Matchup, MatchupStrength, UnitStatistics,
};

pub use system::binding_map::{EffectiveBinding, ResolvedSystemBinding, SystemBindingMap};

pub use unit::grids::{
    CollisionSlots, GridRole, HotkeyCollisionAtCell, HotkeyCollisionCard,
    HotkeyCollisionCardIterator, NamedCommandGrid, PositionCollisionCard,
    PositionCollisionCardIterator, UnitGrids,
};

pub use unit::keyed::{UnitAbilityGroup, UnitAbilitySlot, UnitKeyedCustomKeys};
pub use unit::listing::{
    UnitCategoryEntry, UnitCategoryListing, UnitCategoryRequest, UnitListing, UnitListingEntry,
    UnitListingRequest,
};
pub use unit::slot_containers::UnitSlotContainers;
pub use unit::slots::UnitCommandSlots;

#[cfg(test)]
mod ddd_conformance;

/// Test-only helpers for obtaining object ids. Because `WarcraftObjectId::new`
/// is `pub(crate)` inside `warcraft-api`, no keybinds code — tests included —
/// can fabricate an id from a string; the only way to get one is to ask the
/// database. These wrap `WarcraftApi::default().resolve` so tests name ids by their
/// real string and get the canonical typed id back (panicking on an unknown
/// string, which is exactly the safety guarantee under test).
#[cfg(test)]
pub(crate) mod test_support {
    use crate::identity::slot::GridSlotId;
    use warcraft_api::{WARCRAFT_SYSTEM_KEYBINDS, WarcraftApi, WarcraftObjectId};

    /// Resolve a raw id string to its typed id via the database. Object ids come
    /// from [`WarcraftApi::default().resolve`]; system-keybind section ids (which are
    /// not entries in the object map, e.g. `Ctr1`/`QuickSave`) come from the
    /// system-keybind table. Panics on an unknown string — proving the id was
    /// obtained from the database, never fabricated.
    pub(crate) fn object_id(raw_id: &str) -> WarcraftObjectId {
        WarcraftApi::default()
            .resolve(raw_id)
            .or_else(|| resolve_system_section(raw_id))
            .unwrap_or_else(|| panic!("no known object or system-section id for {raw_id:?}"))
    }

    fn resolve_system_section(raw_id: &str) -> Option<WarcraftObjectId> {
        WARCRAFT_SYSTEM_KEYBINDS
            .iter()
            .map(|system_keybind| system_keybind.section_id())
            .find(|section_id| section_id.value().eq_ignore_ascii_case(raw_id))
    }

    pub(crate) fn ability_slot(raw_id: &str) -> GridSlotId {
        let resolved = object_id(raw_id);
        GridSlotId::ability(resolved)
    }

    pub(crate) fn ability_off_slot(raw_id: &str) -> GridSlotId {
        let resolved = object_id(raw_id);
        GridSlotId::ability_off(resolved)
    }

    pub(crate) fn command_slot(raw_id: &str) -> GridSlotId {
        let resolved = object_id(raw_id);
        GridSlotId::command(resolved)
    }
}
