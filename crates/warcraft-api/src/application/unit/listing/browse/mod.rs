//! The domain answer to "which unit categories and which first unit does a
//! browse produce". The renderer used to call `UnitCatalog::entries_for` and run
//! the dedupe/first-result loop itself at render time; that is a domain decision
//! (ARCHITECTURE R3), so it lives here. The renderer hands over the raw browse
//! inputs and reads back the already-shaped [`UnitListing`].

use crate::application::unit::listing::race_selection::RaceSelection;
use crate::{CatalogVisibility, Scope, SearchField, UnitModeSelection, UnitQuery, WarcraftApi};
use crate::{Race, UnitKind, UnitView, WarcraftObjectId};

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

impl ddd::ReadModel for UnitListing {}

impl ddd::ReadModel for UnitCatalogListing {}

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
}
