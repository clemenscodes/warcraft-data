//! Upgrade domain: upgrade-driven unit replacement. (`UpgradeMeta` joins this
//! module when the metadata concerns are split out.)

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

/// Upgrade metadata: how many research tiers the upgrade has.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct UpgradeMeta {
    max_level: usize,
}

impl UpgradeMeta {
    pub fn new(max_level: usize) -> Self {
        Self { max_level }
    }

    pub fn max_level(&self) -> usize {
        self.max_level
    }
}

// DDD roles: immutable, equality-by-value → Value Objects.
impl ddd::Layered for UnitUpgradeSwap {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for UnitUpgradeSwap {}

impl ddd::Layered for UpgradeMeta {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for UpgradeMeta {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upgrade_meta_stores_max_level() {
        let meta = UpgradeMeta::new(3);
        assert_eq!(meta.max_level(), 3);
    }
}
