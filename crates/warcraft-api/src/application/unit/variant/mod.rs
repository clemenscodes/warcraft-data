//! Variant relation of the `unit` concept: which units are the same logical
//! unit (leveled summon tiers, upgrade-swaps, hero duplicate forms). An
//! application-layer projection over the static database; consumers reach it
//! through `UnitApi` edges (`variants`/`canonical`/`is_variant`) and the
//! `AbilityApi` fanout edge. [`VariantGroup`](group::VariantGroup) is an
//! internal detail, never public.

pub(crate) mod group;
pub(crate) mod registry;
