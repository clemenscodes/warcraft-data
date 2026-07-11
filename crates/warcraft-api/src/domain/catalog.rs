//! Catalog query value objects: the immutable knobs a caller sets when asking
//! the [`UnitCatalog`](crate::UnitCatalog) for a unit listing. Equality-by-value
//! Value Objects with no database access — the query itself lives in the
//! application layer.

/// What a search query is matched against. The sidebar exposes this as a
/// toggle: search units by their own name/id (default), or by the abilities
/// they carry — the latter answers "which units have this ability?" (issue #30,
/// the collision-resolution lookup).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SearchField {
    #[default]
    UnitName,
    Ability,
}

/// Two independent simplifications the sidebar applies while browsing, each
/// separately toggleable. Both default to off (curated browsing — the
/// historical behaviour).
///
/// `include_abilityless_units`: when true, keep units that carry no
/// production, no button-positioned ability, and no shop slot — normally
/// dropped as dead placeholders, but useful for reading a unit's raw stats.
/// This also keeps rally-only buildings (a building with no ability of its
/// own whose only command is the rally point, such as the Demon Gate), which
/// are dead placeholders for hotkey editing for the same reason.
///
/// `expand_variants`: when true, list every member of a variant group
/// (leveled summon tiers, upgrade-swaps, hero duplicates) as its own entry
/// instead of collapsing the group to its strongest canonical member.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CatalogVisibility {
    include_abilityless_units: bool,
    expand_variants: bool,
}

impl CatalogVisibility {
    pub fn new(include_abilityless_units: bool, expand_variants: bool) -> Self {
        Self {
            include_abilityless_units,
            expand_variants,
        }
    }

    pub fn include_abilityless_units(&self) -> bool {
        self.include_abilityless_units
    }

    pub fn expand_variants(&self) -> bool {
        self.expand_variants
    }
}

// DDD roles: catalog-query value objects (equality-by-value).
impl ddd::Layered for SearchField {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for SearchField {}

impl ddd::Layered for CatalogVisibility {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for CatalogVisibility {}
