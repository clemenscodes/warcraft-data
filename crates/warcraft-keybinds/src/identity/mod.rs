//! Identifiers and lookup keys for abilities, slots, and hotkeys.

pub mod ability_id;
pub mod hotkey_target;
pub mod hotkey_token;
pub mod slot;

/// The keyboard keycode value objects live in `warcraft-api`'s keybind domain
/// (`warcraft_api::KeyCode` and its key-category family). They are re-exported
/// here so keybind code keeps importing them from `crate::identity::keycode`.
pub mod keycode {
    pub use warcraft_api::{
        Digit, FunctionKey, KeyCode, KeyCodeOutOfRange, Letter, MouseButton, NotALetter, NumpadKey,
        Punctuation,
    };
}
