//! Ability fanout: which *different-id* abilities on sibling tiers of a variant
//! group must receive the same hotkey/position edit. Derived from the variant
//! groups but an ability concern, so it lives on the ability side. Consumers
//! reach it through the `AbilityApi::fanout` edge.
//!
//! The role-based pairing (`pairing`) is pure; this module is the thin boundary
//! that reads each group member's button-positioned abilities out of the
//! database and memoizes the merged result. The database is threaded in.

pub(crate) mod pairing;

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::application::unit::variant::group::VariantGroup;
use crate::application::unit::variant::variant_index;
use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::WarcraftObjectMeta;
use crate::infrastructure::database::WarcraftDatabase;
use pairing::{AbilityDescriptor, AbilityRoleKey, role_siblings};

/// The built fanout projection: each ability id mapped to the sibling ability
/// ids that must receive the same edit.
pub(crate) struct FanoutIndex {
    siblings: HashMap<WarcraftObjectId, Vec<WarcraftObjectId>>,
}

impl FanoutIndex {
    /// The fanout siblings of `ability_id` — empty for almost every ability.
    pub(crate) fn siblings(&self, ability_id: WarcraftObjectId) -> &[WarcraftObjectId] {
        self.siblings
            .get(&ability_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}

/// The fanout projection over `database`, built once on first use and memoized.
/// Reuses the variant groups (which memoize over the same database).
pub(crate) fn fanout_index(database: &'static WarcraftDatabase) -> &'static FanoutIndex {
    static INDEX: OnceLock<FanoutIndex> = OnceLock::new();
    INDEX.get_or_init(|| build(database, variant_index(database).groups()))
}

/// Pair every group's abilities by role and merge the results into one index.
fn build(database: &WarcraftDatabase, groups: &[VariantGroup]) -> FanoutIndex {
    let mut siblings: HashMap<WarcraftObjectId, Vec<WarcraftObjectId>> = HashMap::new();
    for group in groups {
        let descriptors: Vec<AbilityDescriptor> = group
            .members()
            .iter()
            .flat_map(|member_id| unit_ability_descriptors(database, *member_id))
            .collect();
        for (ability_id, group_siblings) in role_siblings(&descriptors) {
            let entry = siblings.entry(ability_id).or_default();
            for sibling in group_siblings {
                if !entry.contains(&sibling) {
                    entry.push(sibling);
                }
            }
        }
    }
    FanoutIndex { siblings }
}

/// The button-positioned abilities a unit carries (own + hero), each tagged with
/// its role. Abilities without a mechanic code or a default cell can't be paired
/// and are skipped. The one database-reading step of the fanout build.
fn unit_ability_descriptors(
    database: &WarcraftDatabase,
    unit_id: WarcraftObjectId,
) -> Vec<AbilityDescriptor> {
    let Some(object) = database.object(unit_id) else {
        return Vec::new();
    };
    let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
        return Vec::new();
    };
    unit_meta
        .abilities()
        .iter()
        .chain(unit_meta.hero_abilities().iter())
        .filter_map(|ability_id| {
            let ability = database.object(*ability_id)?;
            let WarcraftObjectMeta::Ability(ability_meta) = ability.meta() else {
                return None;
            };
            let code = ability_meta.code()?;
            let position = ability_meta.default_button_position()?;
            Some(AbilityDescriptor {
                ability_id: *ability_id,
                role: AbilityRoleKey {
                    code,
                    column: u8::from(position.column()),
                    row: u8::from(position.row()),
                },
            })
        })
        .collect()
}
