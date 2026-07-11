//! [`CommandView`]: the flat, read-only result of a command query. A command is
//! a standalone in-game action (bindable to a hotkey), independent of any unit.

use crate::domain::command::CommandMeta;
use crate::domain::grid::GridCoordinate;
use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::{WarcraftObject, WarcraftObjectMeta};

/// A single command, as returned by a command query. A `Copy` handle over the
/// immutable stored object.
#[derive(Clone, Copy, Debug)]
pub struct CommandView {
    object: &'static WarcraftObject,
    meta: &'static CommandMeta,
}

impl CommandView {
    pub fn id(&self) -> WarcraftObjectId {
        self.object.id()
    }

    /// The primary display name, if any.
    pub fn name(&self) -> Option<&'static str> {
        self.object.names().first().copied()
    }

    /// The short tooltip, if any.
    pub fn tip(&self) -> Option<&'static str> {
        self.meta.tip()
    }

    /// The command's default grid cell on a command card, if any.
    pub fn default_button_position(&self) -> Option<GridCoordinate> {
        self.meta.default_button_position()
    }
}

/// Views a stored object as a command — succeeds only when it is a command.
impl TryFrom<&'static WarcraftObject> for CommandView {
    type Error = ();

    fn try_from(object: &'static WarcraftObject) -> Result<Self, Self::Error> {
        let WarcraftObjectMeta::Command(meta) = object.meta() else {
            return Err(());
        };
        Ok(Self { object, meta })
    }
}

// DDD role: a read model returned by the command application service.
impl ddd::ReadModel for CommandView {}
