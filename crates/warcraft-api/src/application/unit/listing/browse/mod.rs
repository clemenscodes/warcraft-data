//! The domain answer to "which unit categories and which first unit does a
//! browse produce". The renderer used to call `UnitCatalog::entries_for` and run
//! the dedupe/first-result loop itself at render time; that is a domain decision
//! (ARCHITECTURE R3), so it lives here. The renderer hands over the raw browse
//! inputs and reads back the already-shaped [`UnitListing`].

use crate::application::unit::listing::race_selection::RaceSelection;
use crate::{
    AllRaces, CatalogVisibility, Scope, SearchField, UnitModeSelection, UnitQuery, WarcraftApi,
};
use crate::{Race, UnitKind, UnitMode, UnitView, WarcraftObjectId};
use std::collections::HashSet;

/// The inputs to a unit-list browse: the active race and mode, the current
/// search query and the field it searches, whether hidden units are shown, and
/// the races a search is scoped to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitListingRequest {
    race: Race,
    modes: UnitModeSelection,
    search_query: String,
    search_field: SearchField,
    visibility: CatalogVisibility,
    search_race_scope: RaceSelection,
}

impl UnitListingRequest {
    /// Build a browse request from the current list-view inputs. The search race
    /// scope defaults to every race, so a search behaves as it did before until
    /// [`searching_within`](Self::searching_within) narrows it.
    pub fn new(
        race: Race,
        modes: UnitModeSelection,
        search_query: String,
        search_field: SearchField,
        visibility: CatalogVisibility,
    ) -> Self {
        Self {
            race,
            modes,
            search_query,
            search_field,
            visibility,
            search_race_scope: RaceSelection::All,
        }
    }

    /// Narrow the search to a set of races. A browse ignores this, it always
    /// lists the one active race. The default, when this is not called, is every
    /// race.
    pub fn searching_within(mut self, search_race_scope: RaceSelection) -> Self {
        self.search_race_scope = search_race_scope;
        self
    }

    /// Whether a non-empty search is in effect. A search browses across every
    /// mode, so it drops the mode filter, and across the search race scope, which
    /// defaults to every race.
    fn is_searching(&self) -> bool {
        !self.search_query.is_empty()
    }

    /// The catalog query these inputs describe, across every category.
    ///
    /// A browse narrows to the one active race. A search drops the mode and
    /// applies the search race scope instead, which is every race by default.
    /// That rule is subtle enough that it lives here once rather than in each
    /// caller's `resolve`.
    fn catalog_query(&self) -> UnitQuery<'_> {
        let searching = self.is_searching();
        let races = if searching {
            self.search_race_scope.clone()
        } else {
            RaceSelection::only(self.race)
        };
        let scope = if searching {
            Scope::Search {
                field: self.search_field,
                query: self.search_query.as_str(),
            }
        } else {
            Scope::Browse { modes: self.modes }
        };
        UnitQuery {
            races,
            kind: None,
            visibility: self.visibility,
            scope,
        }
    }
}

/// The first unit a browse yields: its id and its category. The list view
/// selects it when the current selection falls outside the results.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnitListingEntry {
    unit_id: WarcraftObjectId,
    unit_kind: UnitKind,
}

impl UnitListingEntry {
    pub fn unit_id(&self) -> WarcraftObjectId {
        self.unit_id
    }

    pub fn unit_kind(&self) -> UnitKind {
        self.unit_kind
    }
}

/// The shaped result of a unit-list browse: the category kinds present in the
/// results (deduped, in first-seen order) and the first result, if any.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UnitListing {
    category_kinds: Vec<UnitKind>,
    first_result: Option<UnitListingEntry>,
}

impl From<&UnitListingRequest> for UnitListing {
    /// Browse the catalog for the given request and shape the result: walk the
    /// entries once, collecting each distinct category in first-seen order and
    /// remembering the very first entry.
    fn from(request: &UnitListingRequest) -> Self {
        let query = request.catalog_query();
        let entries = WarcraftApi::default().unit().list(&query);
        let mut category_kinds: Vec<UnitKind> = Vec::new();
        let mut first_result: Option<UnitListingEntry> = None;
        for entry in entries {
            let entry_kind = entry.effective_kind();
            if first_result.is_none() {
                let unit_id = entry.id();
                let first_entry = UnitListingEntry {
                    unit_id,
                    unit_kind: entry_kind,
                };
                first_result = Some(first_entry);
            }
            if !category_kinds.contains(&entry_kind) {
                category_kinds.push(entry_kind);
            }
        }
        Self {
            category_kinds,
            first_result,
        }
    }
}

impl UnitListing {
    pub fn category_kinds(&self) -> &[UnitKind] {
        &self.category_kinds
    }

    pub fn first_result(&self) -> Option<&UnitListingEntry> {
        self.first_result.as_ref()
    }
}

/// The inputs to browsing one category's units: the same browse inputs as a
/// [`UnitListingRequest`], narrowed to a single [`UnitKind`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitCategoryRequest {
    race: Race,
    modes: UnitModeSelection,
    category_kind: UnitKind,
    search_query: String,
    search_field: SearchField,
    visibility: CatalogVisibility,
    search_race_scope: RaceSelection,
}

impl UnitCategoryRequest {
    /// Build a per-category browse request. The search race scope defaults to
    /// every race, as [`UnitListingRequest::new`].
    pub fn new(
        race: Race,
        modes: UnitModeSelection,
        category_kind: UnitKind,
        search_query: String,
        search_field: SearchField,
        visibility: CatalogVisibility,
    ) -> Self {
        Self {
            race,
            modes,
            category_kind,
            search_query,
            search_field,
            visibility,
            search_race_scope: RaceSelection::All,
        }
    }

    /// Narrow the search to a set of races, as
    /// [`UnitListingRequest::searching_within`].
    pub fn searching_within(mut self, search_race_scope: RaceSelection) -> Self {
        self.search_race_scope = search_race_scope;
        self
    }

    fn is_searching(&self) -> bool {
        !self.search_query.is_empty()
    }
}

/// One unit in a category browse, shaped for a card: its id and category, its
/// display name (with the "(unnamed)" fallback the domain applies for the rare
/// nameless object), and its icon's database path if it has one. The renderer
/// turns the icon path into a URL — that is presentation, so it stays out here.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnitCategoryEntry {
    unit_id: WarcraftObjectId,
    unit_kind: UnitKind,
    display_name: String,
    icon_database_path: Option<String>,
}

impl UnitCategoryEntry {
    pub fn unit_id(&self) -> WarcraftObjectId {
        self.unit_id
    }

    pub fn unit_kind(&self) -> UnitKind {
        self.unit_kind
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn icon_database_path(&self) -> Option<&str> {
        self.icon_database_path.as_deref()
    }
}

impl From<&UnitView> for UnitCategoryEntry {
    fn from(view: &UnitView) -> Self {
        let names = view.names();
        let first_name = names.first().copied().unwrap_or("(unnamed)");
        let display_name = first_name.to_owned();
        let icons = view.icons();
        let first_icon = icons.first().copied();
        let icon_database_path = first_icon.map(|icon_path| icon_path.to_owned());
        let unit_id = view.id();
        let unit_kind = view.effective_kind();
        Self {
            unit_id,
            unit_kind,
            display_name,
            icon_database_path,
        }
    }
}

/// The units of one category, shaped for the list view's cards.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UnitCategoryListing {
    entries: Vec<UnitCategoryEntry>,
}

impl From<&UnitCategoryRequest> for UnitCategoryListing {
    /// Browse the catalog for one category and shape each entry into card data.
    fn from(request: &UnitCategoryRequest) -> Self {
        let searching = request.is_searching();
        let races = if searching {
            request.search_race_scope.clone()
        } else {
            RaceSelection::only(request.race)
        };
        let query = UnitQuery {
            races,
            kind: Some(request.category_kind),
            visibility: request.visibility,
            scope: if searching {
                Scope::Search {
                    field: request.search_field,
                    query: request.search_query.as_str(),
                }
            } else {
                Scope::Browse {
                    modes: request.modes,
                }
            },
        };
        let catalog_entries = WarcraftApi::default().unit().list(&query);
        let mut entries: Vec<UnitCategoryEntry> = Vec::new();
        for catalog_entry in catalog_entries {
            let entry = UnitCategoryEntry::from(&catalog_entry);
            entries.push(entry);
        }
        Self { entries }
    }
}

impl UnitCategoryListing {
    pub fn entries(&self) -> &[UnitCategoryEntry] {
        &self.entries
    }

    pub fn into_entries(self) -> Vec<UnitCategoryEntry> {
        self.entries
    }
}

/// One category of a catalog browse: the kind, and the units listed under it in
/// the catalog's own order.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UnitCatalogGroup {
    category_kind: UnitKind,
    entries: Vec<UnitCategoryEntry>,
}

impl UnitCatalogGroup {
    pub fn category_kind(&self) -> UnitKind {
        self.category_kind
    }

    pub fn entries(&self) -> &[UnitCategoryEntry] {
        &self.entries
    }

    pub fn into_entries(self) -> Vec<UnitCategoryEntry> {
        self.entries
    }
}

/// A whole browse in one pass: every category with its units, plus the first
/// result.
///
/// This exists because the pair [`UnitListing`] + one [`UnitCategoryListing`]
/// per category asks the same question N+1 times: the caller resolves the
/// categories, then re-scans the whole database once per category to fill it.
/// One pass answers both.
///
/// It also removes a disagreement the split *allows*, even though the bundled
/// database never triggers it (checked exhaustively over every one-, two- and
/// three-letter query): `suppress_fuzzy` runs across the whole candidate set, so
/// a per-category query re-runs it over that category alone and could keep a
/// fuzzy match the full browse dropped. Grouping one pass makes the categories
/// and their contents agree by construction rather than by luck.
///
/// Groups come out in first-seen order. A search sorts by
/// [`UnitKind::search_sort_priority`], which sinks campaign units below melee
/// ones and so lists a kind twice, non-adjacently; the runs are merged back into
/// that kind's one group.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UnitCatalogListing {
    groups: Vec<UnitCatalogGroup>,
    first_result: Option<UnitListingEntry>,
}

impl From<&UnitListingRequest> for UnitCatalogListing {
    /// Browse the catalog once and shape every entry into its category's group.
    fn from(request: &UnitListingRequest) -> Self {
        let query = request.catalog_query();
        let catalog_entries = WarcraftApi::default().unit().list(&query);
        let mut groups: Vec<UnitCatalogGroup> = Vec::new();
        let mut first_result: Option<UnitListingEntry> = None;
        for catalog_entry in catalog_entries {
            let entry = UnitCategoryEntry::from(&catalog_entry);
            let entry_kind = entry.unit_kind();
            if first_result.is_none() {
                let unit_id = entry.unit_id();
                let first_entry = UnitListingEntry {
                    unit_id,
                    unit_kind: entry_kind,
                };
                first_result = Some(first_entry);
            }
            let existing = groups
                .iter_mut()
                .find(|group| group.category_kind == entry_kind);
            match existing {
                Some(group) => group.entries.push(entry),
                None => {
                    let entries = vec![entry];
                    let group = UnitCatalogGroup {
                        category_kind: entry_kind,
                        entries,
                    };
                    groups.push(group);
                }
            }
        }
        Self {
            groups,
            first_result,
        }
    }
}

impl UnitCatalogListing {
    pub fn groups(&self) -> &[UnitCatalogGroup] {
        &self.groups
    }

    pub fn into_groups(self) -> Vec<UnitCatalogGroup> {
        self.groups
    }

    pub fn first_result(&self) -> Option<&UnitListingEntry> {
        self.first_result.as_ref()
    }
}

/// The inputs to a full unit roster: only the visibility toggles. A roster
/// always spans every race in canonical order across both modes, so the race
/// and the mode are fixed rather than inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitRosterRequest {
    visibility: CatalogVisibility,
}

impl UnitRosterRequest {
    /// Build a roster request from the visibility toggles applied to every race
    /// and both modes.
    pub fn new(visibility: CatalogVisibility) -> Self {
        Self { visibility }
    }
}

/// Every unit across all five races and both modes, each listed once, in one
/// fixed domain order: the whole melee block first — race by race in canonical
/// tab order (Human, Orc, Night Elf, Undead, Neutral) — then the whole campaign
/// block, race by race the same way. A unit available in both modes appears once,
/// in the melee block, because that block runs first.
///
/// This exists so a mobile pager can walk every unit in a single, stable order
/// without the renderer composing that order itself — composing the full list
/// across races and modes is a domain decision (ARCHITECTURE R3), so it lives
/// here.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UnitRoster {
    entries: Vec<UnitCategoryEntry>,
}

impl From<&UnitRosterRequest> for UnitRoster {
    /// Walk the whole melee block first — race by race in canonical order — then
    /// the whole campaign block the same way, filing each unit under its own race
    /// the first time it is seen. So every melee unit precedes every campaign
    /// unit, and the last melee unit (a Neutral one) leads into the first campaign
    /// unit (a Human one).
    ///
    /// A race's browse admits every unit that race can reach, which includes the
    /// shared neutral units (tavern heroes, mercenaries, neutral buildings) any
    /// race can use. Keeping those under the first race that reached them would
    /// scatter neutral units through every block and break the race order, so a
    /// unit is filed under its own race only — a shared neutral unit lands in the
    /// Neutral block, once. The `seen` set collapses the melee/campaign overlap so
    /// a unit available in both modes stays in the melee block.
    fn from(request: &UnitRosterRequest) -> Self {
        let visibility = request.visibility;
        let mut seen: HashSet<WarcraftObjectId> = HashSet::new();
        let mut entries: Vec<UnitCategoryEntry> = Vec::new();
        for mode in [UnitMode::Melee, UnitMode::Campaign] {
            for race in AllRaces::ALL.iter() {
                let modes = UnitModeSelection::only(mode);
                let scope = Scope::Browse { modes };
                let query = UnitQuery {
                    races: RaceSelection::only(race),
                    kind: None,
                    visibility,
                    scope,
                };
                let views = WarcraftApi::default().unit().list(&query);
                for view in &views {
                    if view.race() != Some(race) {
                        continue;
                    }
                    if seen.insert(view.id()) {
                        let entry = UnitCategoryEntry::from(view);
                        entries.push(entry);
                    }
                }
            }
        }
        Self { entries }
    }
}

impl UnitRoster {
    pub fn entries(&self) -> &[UnitCategoryEntry] {
        &self.entries
    }

    pub fn into_entries(self) -> Vec<UnitCategoryEntry> {
        self.entries
    }
}

impl ddd::ReadModel for UnitListing {}

impl ddd::ReadModel for UnitCatalogListing {}

impl ddd::ReadModel for UnitRoster {}

#[cfg(test)]
mod tests {
    use super::*;

    fn full_visibility() -> CatalogVisibility {
        CatalogVisibility {
            include_abilityless: true,
            expand_variants: true,
        }
    }

    #[test]
    fn human_melee_browse_yields_categories_and_a_first_result() {
        let request = UnitListingRequest::new(
            Race::Human,
            UnitModeSelection::default(),
            String::new(),
            SearchField::UnitName,
            full_visibility(),
        );
        let listing = UnitListing::from(&request);
        assert!(
            !listing.category_kinds().is_empty(),
            "a human melee browse should surface at least one category"
        );
        let first = listing
            .first_result()
            .expect("a human melee browse should have a first result");
        assert!(!first.unit_id().value().is_empty());
        assert!(
            listing.category_kinds().contains(&first.unit_kind()),
            "the first result's category must appear in the category list"
        );
    }

    #[test]
    fn category_kinds_are_deduped() {
        let request = UnitListingRequest::new(
            Race::Human,
            UnitModeSelection::default(),
            String::new(),
            SearchField::UnitName,
            full_visibility(),
        );
        let listing = UnitListing::from(&request);
        let mut seen = listing.category_kinds().to_vec();
        seen.sort();
        let unique_len = {
            seen.dedup();
            seen.len()
        };
        assert_eq!(
            unique_len,
            listing.category_kinds().len(),
            "category kinds must contain no duplicates"
        );
    }

    fn human_melee_request() -> UnitListingRequest {
        UnitListingRequest::new(
            Race::Human,
            UnitModeSelection::default(),
            String::new(),
            SearchField::UnitName,
            full_visibility(),
        )
    }

    fn search_request(search_query: &str) -> UnitListingRequest {
        let owned_query = search_query.to_owned();
        UnitListingRequest::new(
            Race::Human,
            UnitModeSelection::default(),
            owned_query,
            SearchField::UnitName,
            full_visibility(),
        )
    }

    #[test]
    fn catalog_groups_agree_with_the_listing_categories() {
        let request = human_melee_request();
        let listing = UnitListing::from(&request);
        let catalog = UnitCatalogListing::from(&request);
        let grouped_kinds: Vec<UnitKind> = catalog
            .groups()
            .iter()
            .map(|group| group.category_kind())
            .collect();
        assert_eq!(
            grouped_kinds,
            listing.category_kinds().to_vec(),
            "the grouped catalog must surface exactly the listing's categories, in the same order"
        );
    }

    #[test]
    fn catalog_first_result_agrees_with_the_listing() {
        let request = human_melee_request();
        let listing = UnitListing::from(&request);
        let catalog = UnitCatalogListing::from(&request);
        assert_eq!(
            catalog.first_result(),
            listing.first_result(),
            "both browses walk the same catalog, so they must agree on the first result"
        );
    }

    #[test]
    fn every_group_holds_only_units_of_its_own_category() {
        let request = human_melee_request();
        let catalog = UnitCatalogListing::from(&request);
        assert!(!catalog.groups().is_empty(), "a human melee browse groups");
        for group in catalog.groups() {
            assert!(
                !group.entries().is_empty(),
                "a group must never be empty: an empty group renders as a heading with nothing under it"
            );
            for entry in group.entries() {
                assert_eq!(entry.unit_kind(), group.category_kind());
                assert!(!entry.display_name().is_empty());
            }
        }
    }

    #[test]
    fn no_category_is_grouped_twice() {
        let request = search_request("a");
        let catalog = UnitCatalogListing::from(&request);
        let mut kinds: Vec<UnitKind> = catalog
            .groups()
            .iter()
            .map(|group| group.category_kind())
            .collect();
        let grouped_count = kinds.len();
        kinds.sort();
        kinds.dedup();
        assert_eq!(
            kinds.len(),
            grouped_count,
            "a search sorts campaign units below melee ones, so a kind appears twice \
             non-adjacently in the sort order; those runs must merge into one group"
        );
    }

    #[test]
    fn a_search_keeps_every_matching_unit_across_the_groups() {
        let request = search_request("a");
        let query = request.catalog_query();
        let expected = WarcraftApi::default().unit().list(&query).len();
        let catalog = UnitCatalogListing::from(&request);
        let grouped: usize = catalog
            .groups()
            .iter()
            .map(|group| group.entries().len())
            .sum();
        assert_eq!(
            grouped, expected,
            "grouping must partition the catalog pass, never drop or duplicate an entry"
        );
    }

    #[test]
    fn a_search_scoped_to_one_race_drops_the_other_races_hits() {
        let across_all = search_request("a");
        let human_only = search_request("a").searching_within(RaceSelection::only(Race::Human));
        let all_count: usize = UnitCatalogListing::from(&across_all)
            .groups()
            .iter()
            .map(|group| group.entries().len())
            .sum();
        let human_count: usize = UnitCatalogListing::from(&human_only)
            .groups()
            .iter()
            .map(|group| group.entries().len())
            .sum();
        assert!(
            human_count > 0,
            "a human-scoped search for a common letter must still find human units"
        );
        assert!(
            human_count < all_count,
            "scoping a cross-race search to one race must drop the other races' hits; \
             equal counts would mean the scope was ignored, the bug this fixes"
        );
    }

    #[test]
    fn the_default_search_scope_is_every_race() {
        let implicit = search_request("a");
        let explicit_all = search_request("a").searching_within(RaceSelection::All);
        let human_only = search_request("a").searching_within(RaceSelection::only(Race::Human));
        let count = |request: &UnitListingRequest| -> usize {
            UnitCatalogListing::from(request)
                .groups()
                .iter()
                .map(|group| group.entries().len())
                .sum()
        };
        assert_eq!(
            count(&implicit),
            count(&explicit_all),
            "not narrowing the scope must equal explicitly scoping to every race"
        );
        assert!(
            count(&explicit_all) > count(&human_only),
            "the default scope spans every race, so it finds more than one race can alone"
        );
    }

    #[test]
    fn category_browse_yields_named_entries() {
        let listing_request = UnitListingRequest::new(
            Race::Human,
            UnitModeSelection::default(),
            String::new(),
            SearchField::UnitName,
            full_visibility(),
        );
        let listing = UnitListing::from(&listing_request);
        let category_kind = *listing
            .category_kinds()
            .first()
            .expect("human melee should have at least one category");
        let category_request = UnitCategoryRequest::new(
            Race::Human,
            UnitModeSelection::default(),
            category_kind,
            String::new(),
            SearchField::UnitName,
            full_visibility(),
        );
        let category_listing = UnitCategoryListing::from(&category_request);
        assert!(
            !category_listing.entries().is_empty(),
            "the first category should contain at least one unit"
        );
        for entry in category_listing.entries() {
            assert!(!entry.unit_id().value().is_empty());
            assert!(!entry.display_name().is_empty());
            assert_eq!(entry.unit_kind(), category_kind);
        }
    }

    #[test]
    fn the_roster_lists_every_unit_once_across_all_races_and_modes() {
        let request = UnitRosterRequest::new(full_visibility());
        let roster = UnitRoster::from(&request);
        assert!(
            !roster.entries().is_empty(),
            "a roster spanning every race and both modes must surface units"
        );
        let mut ids: Vec<WarcraftObjectId> = roster
            .entries()
            .iter()
            .map(|entry| entry.unit_id())
            .collect();
        let listed = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(
            ids.len(),
            listed,
            "a unit valid in both modes must appear once, not twice"
        );
        for entry in roster.entries() {
            assert!(!entry.unit_id().value().is_empty());
            assert!(!entry.display_name().is_empty());
        }
    }

    #[test]
    fn the_roster_runs_all_melee_then_all_campaign_each_in_canonical_race_order() {
        let visibility = full_visibility();
        let api = WarcraftApi::default();
        let request = UnitRosterRequest::new(visibility);
        let roster = UnitRoster::from(&request);

        // The units any race reaches in its melee browse, filed under their own
        // race — exactly the ones the roster's melee block should hold.
        let mut melee_ids: HashSet<WarcraftObjectId> = HashSet::new();
        for race in AllRaces::ALL.iter() {
            let modes = UnitModeSelection::only(UnitMode::Melee);
            let scope = Scope::Browse { modes };
            let query = UnitQuery {
                races: RaceSelection::only(race),
                kind: None,
                visibility,
                scope,
            };
            for view in &api.unit().list(&query) {
                if view.race() == Some(race) {
                    melee_ids.insert(view.id());
                }
            }
        }

        let canonical: Vec<Race> = AllRaces::ALL.iter().collect();
        let rank = |race: &Race| {
            canonical
                .iter()
                .position(|candidate| candidate == race)
                .expect("every race has a canonical rank")
        };
        let mut melee_blocks: Vec<Race> = Vec::new();
        let mut campaign_blocks: Vec<Race> = Vec::new();
        let mut in_campaign = false;
        for entry in roster.entries() {
            let race = api
                .object(entry.unit_id())
                .and_then(|object| object.race())
                .expect("every roster unit is filed under its own race, so it has one");
            if melee_ids.contains(&entry.unit_id()) {
                assert!(
                    !in_campaign,
                    "a melee unit follows a campaign unit — the mode blocks are interleaved"
                );
                if melee_blocks.last() != Some(&race) {
                    melee_blocks.push(race);
                }
            } else {
                in_campaign = true;
                if campaign_blocks.last() != Some(&race) {
                    campaign_blocks.push(race);
                }
            }
        }

        assert_eq!(
            melee_blocks, canonical,
            "the melee block runs every race contiguously in canonical order"
        );
        for pair in campaign_blocks.windows(2) {
            assert!(
                rank(&pair[0]) < rank(&pair[1]),
                "the campaign block runs the races it has in canonical order"
            );
        }
    }

    #[test]
    fn the_roster_is_the_deduped_union_of_every_race_and_mode_browse() {
        let visibility = full_visibility();
        let request = UnitRosterRequest::new(visibility);
        let roster = UnitRoster::from(&request);
        let mut expected: HashSet<WarcraftObjectId> = HashSet::new();
        for race in AllRaces::ALL.iter() {
            for mode in [UnitMode::Melee, UnitMode::Campaign] {
                let modes = UnitModeSelection::only(mode);
                let scope = Scope::Browse { modes };
                let query = UnitQuery {
                    races: RaceSelection::only(race),
                    kind: None,
                    visibility,
                    scope,
                };
                for view in &WarcraftApi::default().unit().list(&query) {
                    expected.insert(view.id());
                }
            }
        }
        let listed: HashSet<WarcraftObjectId> = roster
            .entries()
            .iter()
            .map(|entry| entry.unit_id())
            .collect();
        assert_eq!(
            listed, expected,
            "the roster must hold exactly the units the per-race, per-mode browses do"
        );
    }
}
