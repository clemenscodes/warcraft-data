//! Unit listing query: the filter → search → collapse → sort pipeline behind
//! `UnitApi::list`. Each stage is its own module; the pure stages (`search`,
//! `suppress`, `sort`) take only the values they need, the database-driven
//! stages are the boundary composed here.

pub(crate) mod index;
pub(crate) mod placeholder;
pub(crate) mod query;
pub(crate) mod search;
pub(crate) mod search_field;
pub(crate) mod sort;
pub(crate) mod suppress;
pub(crate) mod visibility;

use std::collections::HashSet;

use crate::application::unit::variant::variant_index;
use crate::application::view::unit::UnitView;
use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::{WarcraftObject, WarcraftObjectMeta};
use crate::domain::unit::{UnitKind, UnitMeta, UnitMode};
use crate::infrastructure::database::WarcraftDatabase;
use index::ability_haystack::ability_haystack;
use index::building_ranks::building_ranks;
use index::sold_units::sold_units;
use placeholder::is_placeholder;
use query::{Scope, UnitQuery};
use search::{Match, match_ability, match_unit_name};
use search_field::SearchField;
use sort::sort_key;
use suppress::{any_direct, survives};

/// A unit that passed the filters, tagged with how the search query matched it.
struct Candidate {
    object: &'static WarcraftObject,
    outcome: Match,
}

/// Run a unit listing: filter candidates, suppress fuzzy-only matches when any
/// direct match exists, collapse variant groups, and sort by category then
/// in-game availability then name.
pub(crate) fn list(database: &'static WarcraftDatabase, query: &UnitQuery<'_>) -> Vec<UnitView> {
    let lowercase_query = normalized_query(&query.scope);
    let candidates = candidates(database, query, lowercase_query.as_deref());
    let kept = suppress_fuzzy(candidates);
    let collapsed = collapse_variants(database, kept, query.visibility.expand_variants);
    sorted(database, collapsed, is_search(&query.scope))
}

/// The lowercased, leading-trimmed search query, or `None` for a browse or an
/// empty/whitespace query (which then matches everything).
fn normalized_query(scope: &Scope<'_>) -> Option<String> {
    let Scope::Search { query, .. } = scope else {
        return None;
    };
    let lowercased = query.trim_start().to_ascii_lowercase();
    (!lowercased.trim().is_empty()).then_some(lowercased)
}

/// Whether this is a cross-mode search (vs a mode-filtered browse).
fn is_search(scope: &Scope<'_>) -> bool {
    matches!(scope, Scope::Search { .. })
}

/// Gather every unit that passes the race / mode / placeholder / kind / search
/// gates, tagged with its match outcome.
fn candidates(
    database: &'static WarcraftDatabase,
    query: &UnitQuery<'_>,
    lowercase_query: Option<&str>,
) -> Vec<Candidate> {
    let sold = sold_units(database);
    database
        .iter()
        .filter_map(|(object_id, object)| {
            evaluate(database, *object_id, object, query, lowercase_query, sold)
        })
        .collect()
}

/// Evaluate one object as a listing candidate, or `None` when any gate rejects
/// it.
fn evaluate(
    database: &'static WarcraftDatabase,
    object_id: WarcraftObjectId,
    object: &'static WarcraftObject,
    query: &UnitQuery<'_>,
    lowercase_query: Option<&str>,
    sold: &HashSet<WarcraftObjectId>,
) -> Option<Candidate> {
    let WarcraftObjectMeta::Unit(meta) = object.meta() else {
        return None;
    };
    if let Some(race) = query.race
        && object.race() != Some(race)
    {
        return None;
    }
    if !passes_mode(meta, &query.scope) {
        return None;
    }
    if is_placeholder(database, object_id, meta, sold) && !query.visibility.include_abilityless {
        return None;
    }
    if let Some(kind) = query.kind
        && meta.effective_kind() != kind
    {
        return None;
    }
    if object.names().first().copied().unwrap_or("").is_empty() {
        return None;
    }
    let outcome = match_outcome(database, object_id, object, &query.scope, lowercase_query);
    if outcome == Match::None {
        return None;
    }
    Some(Candidate { object, outcome })
}

/// Whether the unit passes the scope's mode gate: the browsed mode, or either
/// mode for a cross-mode search.
fn passes_mode(meta: &UnitMeta, scope: &Scope<'_>) -> bool {
    match scope {
        Scope::Browse { mode } => meta.passes_filter(*mode),
        Scope::Search { .. } => {
            meta.passes_filter(UnitMode::Melee) || meta.passes_filter(UnitMode::Campaign)
        }
    }
}

/// The search match outcome for a candidate. Browse and empty-query searches
/// keep everything (`Direct`).
fn match_outcome(
    database: &'static WarcraftDatabase,
    object_id: WarcraftObjectId,
    object: &'static WarcraftObject,
    scope: &Scope<'_>,
    lowercase_query: Option<&str>,
) -> Match {
    let (Scope::Search { field, .. }, Some(query)) = (scope, lowercase_query) else {
        return Match::Direct;
    };
    match field {
        SearchField::UnitName => {
            let id_lower = object_id.value().to_ascii_lowercase();
            let names_lower = object
                .names()
                .iter()
                .map(|name| name.to_ascii_lowercase())
                .collect::<Vec<_>>()
                .join(" ");
            match_unit_name(query, &id_lower, &names_lower)
        }
        SearchField::Ability => {
            let haystack = ability_haystack(database)
                .get(&object_id)
                .map(String::as_str)
                .unwrap_or("");
            match_ability(query, haystack)
        }
    }
}

/// Drop fuzzy-only candidates when any candidate matched directly.
fn suppress_fuzzy(candidates: Vec<Candidate>) -> Vec<Candidate> {
    let outcomes: Vec<Match> = candidates
        .iter()
        .map(|candidate| candidate.outcome)
        .collect();
    let any_direct = any_direct(&outcomes);
    candidates
        .into_iter()
        .filter(|candidate| survives(candidate.outcome, any_direct))
        .collect()
}

/// Collapse each variant group to its canonical member (looked up fresh, since
/// the canonical may not have matched the query), deduping so a group surfaces
/// once. With `expand_variants`, list every member under its own id instead.
fn collapse_variants(
    database: &'static WarcraftDatabase,
    candidates: Vec<Candidate>,
    expand_variants: bool,
) -> Vec<UnitView> {
    let index = variant_index(database);
    let mut seen: HashSet<WarcraftObjectId> = HashSet::new();
    let mut views: Vec<UnitView> = Vec::new();
    for candidate in candidates {
        let id = candidate.object.id();
        let display_id = if expand_variants {
            id
        } else {
            index.canonical(id).unwrap_or(id)
        };
        if !seen.insert(display_id) {
            continue;
        }
        if display_id == id {
            if let Ok(view) = UnitView::try_from(candidate.object) {
                views.push(view);
            }
        } else if let Some(view) = database
            .object(display_id)
            .and_then(|object| UnitView::try_from(object).ok())
        {
            views.push(view);
        }
    }
    views
}

/// Sort the surviving views by category priority, in-game availability, name,
/// then id.
fn sorted(
    database: &'static WarcraftDatabase,
    mut views: Vec<UnitView>,
    is_search: bool,
) -> Vec<UnitView> {
    let ranks = building_ranks(database);
    views.sort_by(|left, right| {
        listing_key(database, left, is_search, ranks)
            .cmp(&listing_key(database, right, is_search, ranks))
    });
    views
}

/// The sort key for one view (see [`sort::sort_key`]).
fn listing_key<'a>(
    database: &'static WarcraftDatabase,
    view: &'a UnitView,
    is_search: bool,
    ranks: &std::collections::HashMap<WarcraftObjectId, u32>,
) -> (u8, u32, &'a str, WarcraftObjectId) {
    let id = view.id();
    let (kind, is_campaign, level) = match database.object(id).map(WarcraftObject::meta) {
        Some(WarcraftObjectMeta::Unit(meta)) => {
            (meta.effective_kind(), meta.is_campaign(), meta.level())
        }
        _ => (UnitKind::Soldier, false, 0),
    };
    let building_rank = ranks.get(&id).copied().unwrap_or(0);
    let name = view.name().unwrap_or("");
    sort_key(kind, is_campaign, is_search, building_rank, level, name, id)
}

#[cfg(test)]
mod tests {
    use crate::WarcraftApi;
    use crate::application::unit::listing::query::{Scope, UnitQuery};
    use crate::application::unit::listing::search_field::SearchField;
    use crate::application::unit::listing::visibility::CatalogVisibility;
    use crate::domain::race::Race;
    use crate::domain::unit::{UnitKind, UnitMode};

    fn ids(query: &UnitQuery<'_>) -> Vec<String> {
        WarcraftApi::default()
            .unit()
            .list(query)
            .iter()
            .map(|view| view.id().value().to_string())
            .collect()
    }

    fn browse(race: Race, mode: UnitMode, visibility: CatalogVisibility) -> Vec<String> {
        ids(&UnitQuery {
            race: Some(race),
            visibility,
            scope: Scope::Browse { mode },
            ..UnitQuery::default()
        })
    }

    fn melee(race: Race) -> Vec<String> {
        browse(race, UnitMode::Melee, CatalogVisibility::default())
    }

    fn has(list: &[String], id: &str) -> bool {
        list.iter().any(|entry| entry == id)
    }

    const RALLY_ONLY: [&str; 7] = ["ndmg", "ndke", "ndkw", "ndrb", "ndh3", "ndh4", "nheb"];

    #[test]
    fn rally_only_buildings_hidden_in_curated_browsing() {
        let visible = browse(
            Race::Neutral,
            UnitMode::Campaign,
            CatalogVisibility::default(),
        );
        for placeholder in RALLY_ONLY {
            assert!(!has(&visible, placeholder), "{placeholder} must be hidden");
        }
    }

    #[test]
    fn rally_only_buildings_surface_with_abilityless() {
        let visible = browse(
            Race::Neutral,
            UnitMode::Campaign,
            CatalogVisibility {
                include_abilityless: true,
                expand_variants: false,
            },
        );
        for placeholder in RALLY_ONLY {
            assert!(has(&visible, placeholder), "{placeholder} must surface");
        }
    }

    #[test]
    fn rally_only_gate_keeps_standard_barracks() {
        assert!(has(&melee(Race::Human), "hbar"), "Human Barracks stays");
        assert!(has(&melee(Race::Orc), "obar"), "Orc Barracks stays");
    }

    #[test]
    fn curated_browsing_collapses_variants_to_canonical() {
        let orc = melee(Race::Orc);
        assert!(has(&orc, "osw3") && !has(&orc, "osw1") && !has(&orc, "osw2"));
        assert!(has(&orc, "osp4") && !has(&orc, "osp1"));
        assert!(has(&orc, "otbk") && !has(&orc, "ohun"), "upgrade swap");
        assert!(has(&melee(Race::Human), "hrtt") && !has(&melee(Race::Human), "hmtt"));
    }

    #[test]
    fn heroes_collapse_to_the_produced_hero() {
        let neutral = melee(Race::Neutral);
        assert!(has(&neutral, "Nalc") && !has(&neutral, "Nal2") && !has(&neutral, "Nalm"));
        assert!(has(&neutral, "Ntin") && !has(&neutral, "Nrob"));
        assert!(has(&melee(Race::Human), "Hpal") && !has(&melee(Race::Human), "Huth"));
    }

    #[test]
    fn expand_variants_lists_ability_bearing_members() {
        let expanded = browse(
            Race::Orc,
            UnitMode::Melee,
            CatalogVisibility {
                include_abilityless: false,
                expand_variants: true,
            },
        );
        for wolf in ["osw1", "osw2", "osw3"] {
            assert!(has(&expanded, wolf), "expanded list has {wolf}");
        }
    }

    #[test]
    fn expand_alone_keeps_abilityless_members_gated() {
        let expand_only = CatalogVisibility {
            include_abilityless: false,
            expand_variants: true,
        };
        let expanded = browse(Race::Orc, UnitMode::Melee, expand_only);
        assert!(has(&expanded, "otbk"));
        assert!(!has(&expanded, "ohun"), "ability-less base stays gated");
        let both = browse(
            Race::Orc,
            UnitMode::Melee,
            CatalogVisibility {
                include_abilityless: true,
                expand_variants: true,
            },
        );
        assert!(has(&both, "ohun"), "surfaces once ability-less is on");
    }

    #[test]
    fn include_abilityless_reveals_placeholders_as_a_superset() {
        let revealed = browse(
            Race::Neutral,
            UnitMode::Melee,
            CatalogVisibility {
                include_abilityless: true,
                expand_variants: false,
            },
        );
        let curated = melee(Race::Neutral);
        for id in &curated {
            assert!(has(&revealed, id), "revealed dropped curated {id}");
        }
        for placeholder in ["nanc", "nanw"] {
            assert!(
                !has(&curated, placeholder),
                "{placeholder} hidden by default"
            );
            assert!(has(&revealed, placeholder), "{placeholder} revealed");
        }
    }

    #[test]
    fn neutral_melee_includes_burrow_carriers() {
        let neutral = melee(Race::Neutral);
        assert!(has(&neutral, "nbnb") && has(&neutral, "nanm"));
    }

    #[test]
    fn neutral_melee_excludes_campaign_only_units() {
        let neutral = melee(Race::Neutral);
        for campaign in ["nmyr", "nnsw", "ndrl", "nbel"] {
            assert!(!has(&neutral, campaign), "leaked campaign {campaign}");
        }
    }

    #[test]
    fn search_surfaces_the_canonical_of_a_weaker_variant() {
        let found = ids(&UnitQuery {
            scope: Scope::Search {
                field: SearchField::UnitName,
                query: "osp1",
            },
            ..UnitQuery::default()
        });
        assert!(has(&found, "osp4") && !has(&found, "osp1"));
    }

    #[test]
    fn search_includes_purchasable_units() {
        let found = ids(&UnitQuery {
            scope: Scope::Search {
                field: SearchField::UnitName,
                query: "Ogre Mauler",
            },
            ..UnitQuery::default()
        });
        assert!(has(&found, "nogm"), "purchasable mercenary is findable");
    }

    #[test]
    fn ability_search_lists_carrier_units_by_name_and_id() {
        for query in ["Slow", "ACsw"] {
            let found = ids(&UnitQuery {
                scope: Scope::Search {
                    field: SearchField::Ability,
                    query,
                },
                ..UnitQuery::default()
            });
            for carrier in ["nkog", "nmsn", "nsns"] {
                assert!(has(&found, carrier), "ability '{query}' missing {carrier}");
            }
            assert!(
                !has(&found, "hfoo"),
                "ability '{query}' wrongly included Footman"
            );
        }
    }

    #[test]
    fn unit_name_search_ignores_abilities() {
        let by_ability_name = ids(&UnitQuery {
            scope: Scope::Search {
                field: SearchField::UnitName,
                query: "Slow",
            },
            ..UnitQuery::default()
        });
        assert!(
            !has(&by_ability_name, "nkog"),
            "name search must not surface carriers"
        );
        let by_unit_name = ids(&UnitQuery {
            scope: Scope::Search {
                field: SearchField::UnitName,
                query: "Footman",
            },
            ..UnitQuery::default()
        });
        assert!(has(&by_unit_name, "hfoo"));
    }

    #[test]
    fn soldiers_sort_by_in_game_tech_tier() {
        let soldiers = ids(&UnitQuery {
            race: Some(Race::Human),
            kind: Some(UnitKind::Soldier),
            scope: Scope::Browse {
                mode: UnitMode::Melee,
            },
            ..UnitQuery::default()
        });
        let position = |id: &str| soldiers.iter().position(|entry| entry == id).unwrap();
        assert!(position("hfoo") < position("hrif") && position("hrif") < position("hkni"));
    }

    #[test]
    fn main_hall_upgrade_chains_group_adjacently() {
        let chains = [
            (Race::Human, ["htow", "hkee", "hcas"]),
            (Race::Orc, ["ogre", "ostr", "ofrt"]),
            (Race::Nightelf, ["etol", "etoa", "etoe"]),
            (Race::Undead, ["unpl", "unp1", "unp2"]),
        ];
        for (race, chain) in chains {
            let buildings = ids(&UnitQuery {
                race: Some(race),
                kind: Some(UnitKind::Building),
                scope: Scope::Browse {
                    mode: UnitMode::Melee,
                },
                ..UnitQuery::default()
            });
            let position = |id: &str| buildings.iter().position(|entry| entry == id).unwrap();
            assert_eq!(position(chain[1]), position(chain[0]) + 1, "{race:?}");
            assert_eq!(position(chain[2]), position(chain[1]) + 1, "{race:?}");
        }
    }
}
