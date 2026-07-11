//! [`AbilityApi`]: the application service for the `ability` domain concept.
//! Reached via [`WarcraftApi::ability`](crate::WarcraftApi::ability); it answers
//! ability queries and resolves ability edges, handing back flat views.

use crate::application::ability::fanout::fanout_index;
use crate::application::view::ability::AbilityView;
use crate::application::view::unit::UnitView;
use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::WarcraftObjectMeta;
use crate::infrastructure::database::WarcraftDatabase;

/// Query surface for abilities. A cheap `Copy` handle over the process-wide
/// database; reads through it and returns [`AbilityView`] read models.
#[derive(Clone, Copy, Debug)]
pub struct AbilityApi {
    database: &'static WarcraftDatabase,
}

impl AbilityApi {
    pub(crate) fn new(database: &'static WarcraftDatabase) -> Self {
        Self { database }
    }

    /// The ability with this id, or `None` when the id is unknown or names a
    /// non-ability object.
    pub fn get(&self, id: WarcraftObjectId) -> Option<AbilityView> {
        AbilityView::try_from(self.database.object(id)?).ok()
    }

    /// Every ability in the database.
    pub fn all(&self) -> impl Iterator<Item = AbilityView> {
        let database = self.database;
        database
            .iter()
            .filter_map(|(_id, object)| AbilityView::try_from(object).ok())
    }

    /// The units that carry this ability (as an own or hero ability), resolved
    /// to [`UnitView`]s — the reverse of
    /// [`UnitApi::abilities`](crate::UnitApi::abilities).
    pub fn carriers(&self, id: WarcraftObjectId) -> impl Iterator<Item = UnitView> {
        let database = self.database;
        database.iter().filter_map(move |(_object_id, object)| {
            let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
                return None;
            };
            let carries =
                unit_meta.abilities().contains(&id) || unit_meta.hero_abilities().contains(&id);
            if carries {
                UnitView::try_from(object).ok()
            } else {
                None
            }
        })
    }

    /// The other tier abilities that must receive the same hotkey/position edit
    /// as this ability — its same-role (mechanic + cell), different-id
    /// counterparts on sibling tiers of a variant group. Empty for almost every
    /// ability; non-empty only for different-id tiers like the Carrion Beetle's
    /// Burrow (`Abu2` ↔ `Abu3`).
    pub fn fanout(&self, id: WarcraftObjectId) -> impl Iterator<Item = AbilityView> {
        let database = self.database;
        fanout_index(database)
            .siblings(id)
            .iter()
            .filter_map(move |sibling_id| {
                database
                    .object(*sibling_id)
                    .and_then(|object| AbilityView::try_from(object).ok())
            })
    }
}

// DDD role: the ability application service.
impl ddd::Layered for AbilityApi {
    type Layer = ddd::ApplicationLayer;
}
impl ddd::ApplicationService for AbilityApi {}

#[cfg(test)]
mod tests {
    use crate::WarcraftApi;
    use crate::domain::identity::WarcraftObjectId;

    fn id(value: &'static str) -> WarcraftObjectId {
        WarcraftObjectId::new(value)
    }

    fn fanout_ids(ability: &'static str) -> Vec<WarcraftObjectId> {
        WarcraftApi::default()
            .ability()
            .fanout(id(ability))
            .map(|view| view.id())
            .collect()
    }

    #[test]
    fn carrion_beetle_burrow_fans_out_across_tiers() {
        assert_eq!(fanout_ids("Abu3"), [id("Abu2")]);
        assert_eq!(fanout_ids("Abu2"), [id("Abu3")]);
    }

    #[test]
    fn clockwerk_goblin_self_destruct_fans_out_to_both_siblings() {
        let from_g = fanout_ids("Asdg");
        assert!(from_g.contains(&id("Asd2")));
        assert!(from_g.contains(&id("Asd3")));
        assert!(fanout_ids("Asd2").contains(&id("Asdg")));
    }

    #[test]
    fn shared_id_tier_abilities_have_no_fanout() {
        assert!(fanout_ids("Asal").is_empty());
        assert!(fanout_ids("ACct").is_empty());
    }
}
