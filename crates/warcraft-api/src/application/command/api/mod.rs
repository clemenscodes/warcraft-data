//! [`CommandApi`]: the application service for the `command` domain concept.
//! Reached via [`WarcraftApi::command`](crate::WarcraftApi::command).

use crate::application::view::command::CommandView;
use crate::domain::identity::WarcraftObjectId;
use crate::infrastructure::database::WarcraftDatabase;

/// Query surface for commands. A cheap `Copy` handle over the process-wide
/// database; reads through it and returns [`CommandView`] read models.
#[derive(Clone, Copy, Debug)]
pub struct CommandApi {
    database: &'static WarcraftDatabase,
}

impl CommandApi {
    pub(crate) fn new(database: &'static WarcraftDatabase) -> Self {
        Self { database }
    }

    /// The command with this id, or `None` when the id is unknown or names a
    /// non-command object.
    pub fn get(&self, id: WarcraftObjectId) -> Option<CommandView> {
        CommandView::try_from(self.database.object(id)?).ok()
    }

    /// Every command in the database.
    pub fn all(&self) -> impl Iterator<Item = CommandView> {
        self.database
            .iter()
            .filter_map(|(_id, object)| CommandView::try_from(object).ok())
    }
}

// DDD role: the command application service.
impl ddd::Layered for CommandApi {
    type Layer = ddd::ApplicationLayer;
}
impl ddd::ApplicationService for CommandApi {}
