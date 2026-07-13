//! Keybind domain: the value objects and hand-curated tables that describe how
//! the game's hotkey editor sees commands. One concern per submodule:
//!
//!   - [`keycode`]           — a keyboard key code and its human label.
//!   - [`system_keybind`]    — the system-hotkey definition value objects.
//!   - [`category`]          — grouping of system hotkeys into editor sections.
//!   - [`slots`]             — ordered slot collections for the editor.
//!   - [`mirrors`]           — build/morph command → live-game section mirrors.
//!   - [`ability_tables`]    — ability command-card classification tables.

pub(crate) mod ability_tables;
pub(crate) mod category;
pub(crate) mod keycode;
pub(crate) mod mirrors;
pub(crate) mod slots;
pub(crate) mod system_keybind;
