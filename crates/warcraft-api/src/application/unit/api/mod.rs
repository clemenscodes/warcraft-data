//! [`UnitApi`]: the application service for the `unit` domain concept. Reached
//! via [`WarcraftApi::unit`](crate::WarcraftApi::unit); it answers unit queries
//! and resolves unit edges, handing back flat views.

use crate::application::unit::command_card::command_card;
use crate::application::unit::listing;
use crate::application::unit::listing::query::{Scope, UnitQuery};
use crate::application::unit::variant::variant_index;
use crate::application::view::ability::AbilityView;
use crate::application::view::command::CommandView;
use crate::application::view::unit::UnitView;
use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::WarcraftObjectMeta;
use crate::domain::race::Race;
use crate::domain::statistics::Evasion;
use crate::domain::unit::UnitMode;
use crate::infrastructure::database::WarcraftDatabase;

/// Root and Uproot are the two states of one ability; every uprootable Night Elf
/// building carries a root ability object (`Aro1`/`Aro2`) sharing this base
/// mechanic code. Detecting the code is the 100% signal for "this building
/// uproots" — no hand-maintained id list needed.
const ROOT_ABILITY_CODE: WarcraftObjectId = WarcraftObjectId::new("Aroo");

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
        variant_index(database)
            .group(id)
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
        let canonical_id = variant_index(self.database).canonical(id).unwrap_or(id);
        UnitView::try_from(self.database.object(canonical_id)?).ok()
    }

    /// Whether the unit is a weaker variant the editor hides behind its
    /// canonical sibling.
    pub fn is_variant(&self, id: WarcraftObjectId) -> bool {
        variant_index(self.database).is_hidden(id)
    }

    /// Whether the unit belongs to a variant group at all.
    pub fn has_variants(&self, id: WarcraftObjectId) -> bool {
        variant_index(self.database).group(id).is_some()
    }

    /// Whether the unit is an uprootable building — it carries a Root/Uproot
    /// ability (any id sharing the `Aroo` mechanic code). Catches every root
    /// variant (`Aro1`/`Aro2`), so no hand-maintained id list is needed.
    pub fn can_uproot(&self, id: WarcraftObjectId) -> bool {
        let Some(object) = self.database.object(id) else {
            return false;
        };
        let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
            return false;
        };
        unit_meta.abilities().iter().any(|ability_id| {
            self.database
                .object(*ability_id)
                .and_then(|ability| ability.ability_code())
                == Some(ROOT_ABILITY_CODE)
        })
    }

    /// The ordered command-card buttons the unit shows (movement/attack for
    /// mobile units, rally/tower commands for buildings, a build command for
    /// building workers), as [`CommandView`]s.
    pub fn command_card(&self, id: WarcraftObjectId) -> Vec<CommandView> {
        command_card(self.database, id)
    }

    /// The units matching a listing query — a mode-filtered browse or a
    /// cross-mode search — sorted by category, in-game availability, then name.
    pub fn list(&self, query: &UnitQuery<'_>) -> Vec<UnitView> {
        listing::list(self.database, query)
    }

    /// The highest evasion chance the unit can field, across its standard and
    /// hero abilities at each ability's full level. Evasion abilities do not
    /// stack — the strongest wins — so this is the unit's dodge chance.
    /// [`Evasion::default`] (a chance of zero) when the id is unknown, names a
    /// non-unit, or the unit has no evasion source. Scanning the ability catalog
    /// for the granted chance is application work, so it lives here; the derived
    /// figure it feeds ([`UnitStatistics`](crate::UnitStatistics)) stays pure.
    pub fn evasion(&self, unit_id: WarcraftObjectId) -> Evasion {
        let Some(object) = self.database.object(unit_id) else {
            return Evasion::default();
        };
        let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
            return Evasion::default();
        };
        let mut best_chance: f32 = 0.0;
        for ability_id in unit_meta
            .abilities()
            .iter()
            .chain(unit_meta.hero_abilities().iter())
        {
            let Some(ability_object) = self.database.object(*ability_id) else {
                continue;
            };
            let WarcraftObjectMeta::Ability(ability_meta) = ability_object.meta() else {
                continue;
            };
            for chance in ability_meta.evasion_chances() {
                let fraction = chance.as_fraction();
                if fraction > best_chance {
                    best_chance = fraction;
                }
            }
        }
        Evasion::new(best_chance)
    }

    /// The natural default selection for a race/mode browse: the first unit a
    /// curated listing would show.
    pub fn default_unit(&self, race: Race, mode: UnitMode) -> Option<UnitView> {
        self.list(&UnitQuery {
            race: Some(race),
            scope: Scope::Browse { mode },
            ..UnitQuery::default()
        })
        .into_iter()
        .next()
    }
}

// DDD role: the unit application service.
impl ddd::Layered for UnitApi {
    type Layer = ddd::ApplicationLayer;
}
impl ddd::ApplicationService for UnitApi {}

#[cfg(test)]
mod tests {
    use crate::WarcraftApi;
    use crate::domain::identity::WarcraftObjectId;

    fn id(value: &'static str) -> WarcraftObjectId {
        WarcraftObjectId::new(value)
    }

    fn variant_ids(unit: &'static str) -> Vec<WarcraftObjectId> {
        WarcraftApi::default()
            .unit()
            .variants(id(unit))
            .map(|view| view.id())
            .collect()
    }

    fn canonical_id(unit: &'static str) -> Option<WarcraftObjectId> {
        WarcraftApi::default()
            .unit()
            .canonical(id(unit))
            .map(|view| view.id())
    }

    #[test]
    fn leveled_summon_tiers_collapse_to_strongest() {
        assert_eq!(variant_ids("osw1"), [id("osw1"), id("osw2"), id("osw3")]);
        assert_eq!(canonical_id("osw1"), Some(id("osw3")));
        assert_eq!(canonical_id("osw2"), Some(id("osw3")));
        let api = WarcraftApi::default();
        assert!(api.unit().is_variant(id("osw1")));
        assert!(api.unit().is_variant(id("osw2")));
        assert!(!api.unit().is_variant(id("osw3")));
    }

    #[test]
    fn overlapping_summon_chains_union_merge() {
        assert_eq!(
            variant_ids("nqb1"),
            [id("nqb1"), id("nqb2"), id("nqb3"), id("nqb4")]
        );
        assert_eq!(canonical_id("nqb3"), Some(id("nqb4")));
        assert_eq!(
            variant_ids("osp1"),
            [id("osp1"), id("osp2"), id("osp3"), id("osp4")]
        );
        assert_eq!(canonical_id("osp1"), Some(id("osp4")));
    }

    #[test]
    fn upgrade_swaps_collapse_to_the_upgraded_unit() {
        assert_eq!(canonical_id("ohun"), Some(id("otbk")));
        assert_eq!(canonical_id("hmtt"), Some(id("hrtt")));
        let api = WarcraftApi::default();
        assert!(api.unit().is_variant(id("ohun")));
        assert!(!api.unit().is_variant(id("otbk")));
        assert!(api.unit().is_variant(id("hmtt")));
    }

    #[test]
    fn curated_tiers_collapse_to_strongest() {
        assert_eq!(variant_ids("ucs2"), [id("ucs1"), id("ucs2"), id("ucs3")]);
        assert_eq!(canonical_id("ucs1"), Some(id("ucs3")));
        assert_eq!(variant_ids("ucsB"), [id("ucsB"), id("ucsC")]);
        assert_eq!(canonical_id("ucsB"), Some(id("ucsC")));
        assert_eq!(
            variant_ids("ncg1"),
            [id("ncg1"), id("ncg2"), id("ncg3"), id("ncgb")]
        );
        assert_eq!(canonical_id("ncg1"), Some(id("ncgb")));
    }

    #[test]
    fn heroes_collapse_to_the_produced_hero() {
        for weaker in ["Nal2", "Nal3", "Nalm"] {
            assert_eq!(canonical_id(weaker), Some(id("Nalc")));
            assert!(WarcraftApi::default().unit().is_variant(id(weaker)));
        }
        assert!(!WarcraftApi::default().unit().is_variant(id("Nalc")));
        assert_eq!(canonical_id("Nrob"), Some(id("Ntin")));
        assert_eq!(canonical_id("Huth"), Some(id("Hpal")));
    }

    #[test]
    fn non_unit_false_positives_never_form_a_group() {
        let api = WarcraftApi::default();
        for false_positive in ["Rguv", "Reuv"] {
            assert!(!api.unit().has_variants(id(false_positive)));
            assert!(!api.unit().is_variant(id(false_positive)));
        }
    }

    #[test]
    fn a_standalone_unit_is_its_own_canonical_with_no_variants() {
        // The Footman belongs to no variant group.
        assert!(variant_ids("hfoo").is_empty());
        assert_eq!(canonical_id("hfoo"), Some(id("hfoo")));
        assert!(!WarcraftApi::default().unit().is_variant(id("hfoo")));
    }

    #[test]
    fn can_attack_reflects_the_attacking_building_list() {
        let api = WarcraftApi::default();
        assert!(
            api.unit()
                .get(id("hgtw"))
                .expect("guard tower")
                .can_attack()
        );
        assert!(!api.unit().get(id("htow")).expect("town hall").can_attack());
    }

    #[test]
    fn a_unit_without_an_evasion_ability_resolves_to_zero() {
        let footman_evasion = WarcraftApi::default().unit().evasion(id("hfoo")).chance();
        assert_eq!(footman_evasion, 0.0);
    }

    #[test]
    fn a_hero_with_evasion_resolves_a_positive_chance() {
        let demon_hunter_evasion = WarcraftApi::default().unit().evasion(id("Edem")).chance();
        assert!(demon_hunter_evasion > 0.0);
    }

    #[test]
    fn can_uproot_detects_the_root_ability_code() {
        let api = WarcraftApi::default();
        assert!(api.unit().can_uproot(id("etol")), "Tree of Life uproots");
        assert!(!api.unit().can_uproot(id("hbar")), "Barracks does not");
        // etrp/ncap carry the Aro2 variant (different id, same Aroo code).
        assert!(api.unit().can_uproot(id("etrp")));
        for corrupted in ["nctl", "ncta", "ncte", "ncaw", "ncap"] {
            assert!(
                api.unit().can_uproot(id(corrupted)),
                "corrupted building {corrupted} must uproot",
            );
        }
    }
}
