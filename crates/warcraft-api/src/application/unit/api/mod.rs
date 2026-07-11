//! [`UnitApi`]: the application service for the `unit` domain concept. Reached
//! via [`WarcraftApi::unit`](crate::WarcraftApi::unit); it answers unit queries
//! and resolves unit edges, handing back flat views.

use crate::application::unit::variant::registry;
use crate::application::view::ability::AbilityView;
use crate::application::view::unit::UnitView;
use crate::domain::identity::WarcraftObjectId;
use crate::infrastructure::database::WarcraftDatabase;

/// Query surface for units. A cheap `Copy` handle over the process-wide
/// database; reads through it and returns [`UnitView`] read models.
#[derive(Clone, Copy, Debug)]
pub struct UnitApi {
    database: &'static WarcraftDatabase,
}

impl UnitApi {
    pub(crate) fn new(database: &'static WarcraftDatabase) -> Self {
        Self { database }
    }

    /// The unit with this id, or `None` when the id is unknown or names a
    /// non-unit object.
    pub fn get(&self, id: WarcraftObjectId) -> Option<UnitView> {
        UnitView::try_from(self.database.object(id)?).ok()
    }

    /// Every unit in the database.
    pub fn all(&self) -> impl Iterator<Item = UnitView> {
        let database = self.database;
        database
            .iter()
            .filter_map(|(_id, object)| UnitView::try_from(object).ok())
    }

    /// Resolve the abilities the unit with this id carries (its own and its
    /// hero abilities) to [`AbilityView`]s. Unknown ability ids are skipped.
    pub fn abilities(&self, id: WarcraftObjectId) -> impl Iterator<Item = AbilityView> {
        let database = self.database;
        let unit = database
            .object(id)
            .and_then(|object| UnitView::try_from(object).ok());
        unit.into_iter()
            .flat_map(|unit| {
                unit.ability_ids()
                    .iter()
                    .chain(unit.hero_ability_ids().iter())
            })
            .filter_map(move |ability_id| {
                database
                    .object(*ability_id)
                    .and_then(|object| AbilityView::try_from(object).ok())
            })
    }

    /// The other forms of the same logical unit — leveled summon tiers,
    /// upgrade-swaps, hero duplicate forms — as [`UnitView`]s. Empty when the
    /// unit stands alone (the common case).
    pub fn variants(&self, id: WarcraftObjectId) -> impl Iterator<Item = UnitView> {
        let database = self.database;
        registry::variant_group(id)
            .into_iter()
            .flat_map(|group| group.members().iter())
            .filter_map(move |member_id| {
                database
                    .object(*member_id)
                    .and_then(|object| UnitView::try_from(object).ok())
            })
    }

    /// The canonical form of the unit's variant group — the strongest tier /
    /// upgraded unit / produced hero. Returns the unit itself when it already is
    /// the canonical or stands alone.
    pub fn canonical(&self, id: WarcraftObjectId) -> Option<UnitView> {
        let canonical_id = registry::canonical(id).unwrap_or(id);
        UnitView::try_from(self.database.object(canonical_id)?).ok()
    }

    /// Whether the unit is a weaker variant the editor hides behind its
    /// canonical sibling.
    pub fn is_variant(&self, id: WarcraftObjectId) -> bool {
        registry::is_hidden_variant(id)
    }

    /// Whether the unit belongs to a variant group at all.
    pub fn has_variants(&self, id: WarcraftObjectId) -> bool {
        registry::variant_group(id).is_some()
    }
}

// DDD role: the unit application service.
impl ddd::Layered for UnitApi {
    type Layer = ddd::ApplicationLayer;
}
impl ddd::ApplicationService for UnitApi {}
