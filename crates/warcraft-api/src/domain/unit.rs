//! Unit domain: unit taxonomy plus the production/flags value objects. The
//! float-bearing unit stats (`UnitAttack`, `UnitCombat`, `UnitMeta`) join this
//! module in slice 3, together with their fixed-point quantity conversion.

use crate::domain::identity::WarcraftObjectId;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitKind {
    #[default]
    Soldier,
    Worker,
    Hero,
    Building,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitFlags {
    is_campaign: bool,
    is_in_editor: bool,
    is_hidden_in_editor: bool,
    is_special: bool,
}

impl UnitFlags {
    pub const EDITOR_ONLY: UnitFlags = UnitFlags {
        is_campaign: false,
        is_in_editor: true,
        is_hidden_in_editor: false,
        is_special: false,
    };

    pub const fn new(
        is_campaign: bool,
        is_in_editor: bool,
        is_hidden_in_editor: bool,
        is_special: bool,
    ) -> Self {
        Self {
            is_campaign,
            is_in_editor,
            is_hidden_in_editor,
            is_special,
        }
    }

    pub const fn is_campaign(&self) -> bool {
        self.is_campaign
    }

    pub const fn is_in_editor(&self) -> bool {
        self.is_in_editor
    }

    pub const fn is_hidden_in_editor(&self) -> bool {
        self.is_hidden_in_editor
    }

    pub const fn is_special(&self) -> bool {
        self.is_special
    }
}

// Mirrors the `regenType` column in `unitbalance.slk`. Controls when HP
// regeneration is active; the rate (`hit_points_regen`) is the per-second
// value WHILE active, with no day/night multiplier on top of it.
//
//   Always — regenerates anywhere, anytime (Human, Orc, neutral creeps).
//   Night  — regenerates only between dusk and dawn (Night Elf).
//   Blight — regenerates only while standing on blight (Undead).
//   None   — does not regenerate HP at all (some neutral structures /
//            mechanical creeps).
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegenType {
    #[default]
    Always,
    Night,
    Blight,
    None,
}

impl RegenType {
    pub fn parse(raw_value: &str) -> Self {
        match raw_value.trim().to_ascii_lowercase().as_str() {
            "always" => Self::Always,
            "night" => Self::Night,
            "blight" => Self::Blight,
            "none" | "" | "-" | "_" => Self::None,
            _ => Self::Always,
        }
    }
}

// DDD roles: immutable, equality-by-value → Value Objects.
impl ddd::Layered for UnitKind {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for UnitKind {}

impl ddd::Layered for UnitProduction {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for UnitProduction {}

impl ddd::Layered for UnitFlags {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for UnitFlags {}

impl ddd::Layered for RegenType {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for RegenType {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regen_type_parse_known_values() {
        assert_eq!(RegenType::parse("always"), RegenType::Always);
        assert_eq!(RegenType::parse("night"), RegenType::Night);
        assert_eq!(RegenType::parse("blight"), RegenType::Blight);
        assert_eq!(RegenType::parse("none"), RegenType::None);
    }

    #[test]
    fn regen_type_parse_empty_and_dash_are_none() {
        assert_eq!(RegenType::parse(""), RegenType::None);
        assert_eq!(RegenType::parse("-"), RegenType::None);
        assert_eq!(RegenType::parse("_"), RegenType::None);
    }
}

/// The game mode a unit list is filtered for: standard melee, or campaign.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnitMode {
    Melee,
    Campaign,
}

impl TryFrom<&str> for UnitMode {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "melee" => Ok(Self::Melee),
            "campaign" => Ok(Self::Campaign),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for UnitMode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Melee => formatter.write_str("melee"),
            Self::Campaign => formatter.write_str("campaign"),
        }
    }
}

impl ddd::Layered for UnitMode {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for UnitMode {}
