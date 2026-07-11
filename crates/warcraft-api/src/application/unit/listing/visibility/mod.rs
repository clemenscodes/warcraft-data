//! [`CatalogVisibility`]: the two independent simplifications a caller can toggle
//! while browsing. Both default off (curated browsing). A query input, so it
//! lives in the application layer with named fields — never a positional
//! `(bool, bool)`.

/// Two orthogonal listing toggles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CatalogVisibility {
    /// Keep units normally dropped as dead placeholders — those with no
    /// production, no button-positioned ability, and no shop slot — plus
    /// rally-only buildings. The sole gate for both placeholder kinds.
    pub include_abilityless: bool,
    /// List every member of a variant group as its own entry instead of
    /// collapsing the group to its canonical member.
    pub expand_variants: bool,
}
