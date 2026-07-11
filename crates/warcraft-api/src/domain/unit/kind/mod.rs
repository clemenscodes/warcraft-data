//! [`UnitKind`]: the taxonomy a unit falls under (soldier / worker / hero /
//! building).

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitKind {
    #[default]
    Soldier,
    Worker,
    Hero,
    Building,
}

impl UnitKind {
    /// The heading a unit of this kind sorts under in a catalog listing.
    pub fn category_label(self) -> &'static str {
        match self {
            UnitKind::Hero => "Heroes",
            UnitKind::Soldier => "Units",
            UnitKind::Worker => "Workers",
            UnitKind::Building => "Buildings",
        }
    }

    /// Relative order of this kind's category within a listing (lower first).
    pub fn category_priority(self) -> u8 {
        match self {
            UnitKind::Hero => 0,
            UnitKind::Building => 1,
            UnitKind::Worker => 2,
            UnitKind::Soldier => 3,
        }
    }

    /// Sort priority for search results, sinking campaign-only units below their
    /// melee counterparts while preserving the per-kind category order.
    pub fn search_sort_priority(self, is_campaign: bool) -> u8 {
        match (is_campaign, self) {
            (false, UnitKind::Hero) => 0,
            (false, UnitKind::Building) => 1,
            (false, UnitKind::Worker) => 2,
            (false, UnitKind::Soldier) => 3,
            (true, UnitKind::Hero) => 4,
            (true, UnitKind::Building) => 5,
            (true, UnitKind::Worker) => 6,
            (true, UnitKind::Soldier) => 7,
        }
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for UnitKind {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for UnitKind {}
