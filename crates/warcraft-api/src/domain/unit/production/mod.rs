//! [`UnitProduction`]: what a unit can research, build, train, or sell.

use crate::domain::identity::WarcraftObjectId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitProduction {
    researches: &'static [WarcraftObjectId],
    builds: &'static [WarcraftObjectId],
    trains: &'static [WarcraftObjectId],
    sell_items: &'static [WarcraftObjectId],
    sell_units: &'static [WarcraftObjectId],
}

impl UnitProduction {
    pub const EMPTY: UnitProduction = UnitProduction {
        researches: &[],
        builds: &[],
        trains: &[],
        sell_items: &[],
        sell_units: &[],
    };

    pub const fn new(
        researches: &'static [WarcraftObjectId],
        builds: &'static [WarcraftObjectId],
        trains: &'static [WarcraftObjectId],
        sell_items: &'static [WarcraftObjectId],
        sell_units: &'static [WarcraftObjectId],
    ) -> Self {
        Self {
            researches,
            builds,
            trains,
            sell_items,
            sell_units,
        }
    }

    pub const fn researches(&self) -> &'static [WarcraftObjectId] {
        self.researches
    }

    pub const fn builds(&self) -> &'static [WarcraftObjectId] {
        self.builds
    }

    pub const fn trains(&self) -> &'static [WarcraftObjectId] {
        self.trains
    }

    pub const fn sell_items(&self) -> &'static [WarcraftObjectId] {
        self.sell_items
    }

    pub const fn sell_units(&self) -> &'static [WarcraftObjectId] {
        self.sell_units
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for UnitProduction {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for UnitProduction {}
