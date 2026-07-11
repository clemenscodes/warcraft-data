//! Per-unit lowercase search text built from the names and ids of the abilities
//! the unit carries on a command button. Lets an ability search match a unit by
//! the abilities it carries. Leading/trailing spaces make whole-word token
//! matching work. Only button-positioned abilities are included — the same gate
//! the placeholder rule uses for "has a visible ability".

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::WarcraftObjectMeta;
use crate::infrastructure::database::WarcraftDatabase;

/// The memoized per-unit ability haystack. Units with no button-positioned
/// abilities are absent.
pub(crate) fn ability_haystack(
    database: &'static WarcraftDatabase,
) -> &'static HashMap<WarcraftObjectId, String> {
    static HAYSTACK: OnceLock<HashMap<WarcraftObjectId, String>> = OnceLock::new();
    HAYSTACK.get_or_init(|| {
        let mut haystacks: HashMap<WarcraftObjectId, String> = HashMap::new();
        for (object_id, object) in database.iter() {
            let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
                continue;
            };
            let haystack = unit_haystack(database, unit_meta);
            if !haystack.is_empty() {
                haystacks.insert(*object_id, haystack);
            }
        }
        haystacks
    })
}

/// The haystack text for one unit: for each button-positioned ability it
/// carries, its lowercased names then its lowercased id, space-separated and
/// space-terminated.
fn unit_haystack(database: &WarcraftDatabase, unit_meta: &crate::domain::unit::UnitMeta) -> String {
    let mut haystack = String::new();
    let ability_ids = unit_meta
        .abilities()
        .iter()
        .chain(unit_meta.hero_abilities().iter());
    for ability_id in ability_ids {
        let Some(ability) = database.object(*ability_id) else {
            continue;
        };
        let WarcraftObjectMeta::Ability(ability_meta) = ability.meta() else {
            continue;
        };
        if ability_meta.default_button_position().is_none() {
            continue;
        }
        for ability_name in ability.names() {
            haystack.push(' ');
            haystack.push_str(&ability_name.to_ascii_lowercase());
        }
        haystack.push(' ');
        haystack.push_str(&ability_id.value().to_ascii_lowercase());
    }
    if !haystack.is_empty() {
        haystack.push(' ');
    }
    haystack
}
