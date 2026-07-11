//! Variant relation of the `unit` concept: which units are the same logical
//! unit (leveled summon tiers, upgrade-swaps, hero duplicate forms). A read-only
//! projection built once; consumers reach it through the `UnitApi` variant
//! edges. [`VariantGroup`](group::VariantGroup) and
//! [`VariantIndex`](index::VariantIndex) are internal, never public.
//!
//! Layering: `facts` (database → minimal id sets) → `chains` (sets + tables →
//! evidence chains) → `build` (chains → ordered groups, via `union_find`) →
//! `index` (groups → queryable lookups). Every stage is pure over its arguments;
//! the composition root below is the one place that names the global data.

pub(crate) mod build;
pub(crate) mod chains;
pub(crate) mod facts;
pub(crate) mod group;
pub(crate) mod index;
pub(crate) mod union_find;

use std::sync::OnceLock;

use crate::domain::identity::WarcraftObjectId;
use crate::infrastructure::database::WarcraftDatabase;
use crate::{TIERED_UNIT_GROUPS, UNIT_UPGRADE_SWAPS};
use index::VariantIndex;

/// Tiered units the game data does *not* link authoritatively: their summon
/// ability omits the tier unit ids, no upgrade swaps them, and nothing
/// references them — only the shared name and id-suffix relate them. Hand-curated
/// (with the project owner's sign-off) because the set is tiny and stable, and a
/// verified id list is reliable where a name heuristic would not be. Each entry
/// is ordered weakest → strongest.
///
/// - Carrion Beetle (`ucs2` carries Burrow `Abu2`, `ucs3` carries `Abu3`).
/// - Burrowed Carrion Beetle (`ucsB`/`ucsC`).
/// - Clockwerk Goblin (`ncg1`/`ncg2`/`ncg3`/`ncgb`), the Pocket Factory's four
///   stat-identical forms differing only in Self Destruct id.
const CURATED_TIER_GROUPS: &[&[WarcraftObjectId]] = &[
    &[
        WarcraftObjectId::new("ucs1"),
        WarcraftObjectId::new("ucs2"),
        WarcraftObjectId::new("ucs3"),
    ],
    &[WarcraftObjectId::new("ucsB"), WarcraftObjectId::new("ucsC")],
    &[
        WarcraftObjectId::new("ncg1"),
        WarcraftObjectId::new("ncg2"),
        WarcraftObjectId::new("ncg3"),
        WarcraftObjectId::new("ncgb"),
    ],
];

/// The variant projection over `database`, built once on first use and memoized.
/// The database is passed in by the caller (`UnitApi`, which holds it); this is
/// the sole boundary that names the generated data tables and injects them into
/// the pure build.
pub(crate) fn variant_index(database: &'static WarcraftDatabase) -> &'static VariantIndex {
    static INDEX: OnceLock<VariantIndex> = OnceLock::new();
    INDEX.get_or_init(|| {
        let facts = facts::extract(database);
        let tier_groups: Vec<&[WarcraftObjectId]> =
            [TIERED_UNIT_GROUPS, CURATED_TIER_GROUPS].concat();
        let upgrade_swaps: Vec<(WarcraftObjectId, WarcraftObjectId)> = UNIT_UPGRADE_SWAPS
            .iter()
            .map(|swap| (swap.from_unit_id(), swap.to_unit_id()))
            .collect();
        build::groups(&chains::chains(&tier_groups, &upgrade_swaps, &facts)).into()
    })
}
