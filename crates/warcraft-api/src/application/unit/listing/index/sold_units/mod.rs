//! The set of unit ids that some shop sells. A purchasable unit (e.g. the Ogre
//! Mauler `nogm`) carries no abilities or production of its own — it only ever
//! appears as a shop buy-button — so the listing keeps it via this reverse
//! lookup instead of dropping it as a placeholder.

use std::collections::HashSet;
use std::sync::OnceLock;

use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::WarcraftObjectMeta;
use crate::infrastructure::database::WarcraftDatabase;

/// Every unit id appearing in some unit's `sell_units` list, memoized.
pub(crate) fn sold_units(
    database: &'static WarcraftDatabase,
) -> &'static HashSet<WarcraftObjectId> {
    static SOLD: OnceLock<HashSet<WarcraftObjectId>> = OnceLock::new();
    SOLD.get_or_init(|| {
        let mut sold: HashSet<WarcraftObjectId> = HashSet::new();
        for (_object_id, object) in database.iter() {
            let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
                continue;
            };
            for sold_id in unit_meta.sell_units() {
                sold.insert(*sold_id);
            }
        }
        sold
    })
}
