//! [`CommandMeta`]: a command-card command button's default grid position and
//! tooltips.

use crate::domain::grid::GridCoordinate;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandMeta {
    default_button_position: Option<GridCoordinate>,
    tip: Option<&'static str>,
    ubertip: Option<&'static str>,
}

impl CommandMeta {
    pub const fn new(default_button_position: Option<GridCoordinate>) -> Self {
        Self {
            default_button_position,
            tip: None,
            ubertip: None,
        }
    }

    pub const fn with_text(
        default_button_position: Option<GridCoordinate>,
        tip: Option<&'static str>,
        ubertip: Option<&'static str>,
    ) -> Self {
        Self {
            default_button_position,
            tip,
            ubertip,
        }
    }

    pub fn default_button_position(&self) -> Option<GridCoordinate> {
        self.default_button_position
    }

    pub fn tip(&self) -> Option<&'static str> {
        self.tip
    }

    pub fn ubertip(&self) -> Option<&'static str> {
        self.ubertip
    }
}

// DDD role: immutable command metadata, equality-by-value → Value Object.
impl ddd::Layered for CommandMeta {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for CommandMeta {}
