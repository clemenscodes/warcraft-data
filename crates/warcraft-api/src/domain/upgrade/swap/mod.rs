//! [`UnitUpgradeSwap`]: an upgrade whose effect replaces one trained unit with
//! another.

use crate::domain::identity::WarcraftObjectId;

/// An upgrade-swap: an upgrade whose effect replaces one trained unit with
/// another (e.g. Berserker upgrading Headhunters `ohun` into Berserkers
/// `otbk`). Extracted from `units/upgradedata.slk` rows whose effect slot is
/// the `rtma` "replace unit" mechanic.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitUpgradeSwap {
    from_unit_id: WarcraftObjectId,
    to_unit_id: WarcraftObjectId,
}

impl UnitUpgradeSwap {
    pub(crate) const fn new(from_unit_id: &'static str, to_unit_id: &'static str) -> Self {
        let from_unit_id = WarcraftObjectId::new(from_unit_id);
        let to_unit_id = WarcraftObjectId::new(to_unit_id);
        Self {
            from_unit_id,
            to_unit_id,
        }
    }

    pub fn from_unit_id(&self) -> WarcraftObjectId {
        self.from_unit_id
    }

    pub fn to_unit_id(&self) -> WarcraftObjectId {
        self.to_unit_id
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for UnitUpgradeSwap {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for UnitUpgradeSwap {}
