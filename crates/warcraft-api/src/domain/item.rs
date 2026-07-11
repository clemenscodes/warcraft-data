//! Item domain: an item's classification and item metadata.

use serde::{Deserialize, Serialize};

use crate::domain::identity::WarcraftObjectId;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemClass {
    Permanent = 0x0,
    Charged = 0x1,
    PowerUp = 0x2,
    Artifact = 0x3,
    #[default]
    Purchasable = 0x4,
    Campaign = 0x5,
    Miscellaneous = 0x6,
    Unknown = 0x7,
    Any = 0x8,
}

impl TryFrom<&str> for ItemClass {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Artifact" => Ok(ItemClass::Artifact),
            "Permanent" => Ok(ItemClass::Permanent),
            "Charged" => Ok(ItemClass::Charged),
            "PowerUp" => Ok(ItemClass::PowerUp),
            "Campaign" => Ok(ItemClass::Campaign),
            "Miscellaneous" => Ok(ItemClass::Miscellaneous),
            "Purchasable" => Ok(ItemClass::Purchasable),
            _ => Err(()),
        }
    }
}

impl Serialize for ItemClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let name = match self {
            ItemClass::Permanent => "Permanent",
            ItemClass::Charged => "Charged",
            ItemClass::PowerUp => "PowerUp",
            ItemClass::Artifact => "Artifact",
            ItemClass::Purchasable => "Purchasable",
            ItemClass::Campaign => "Campaign",
            ItemClass::Miscellaneous => "Miscellaneous",
            ItemClass::Unknown => "Unknown",
            ItemClass::Any => "Any",
        };
        serializer.serialize_str(name)
    }
}

impl<'de> Deserialize<'de> for ItemClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        use ItemClass::*;
        Ok(match value {
            0x0 => Permanent,
            0x1 => Charged,
            0x2 => PowerUp,
            0x3 => Artifact,
            0x4 => Purchasable,
            0x5 => Campaign,
            0x6 => Miscellaneous,
            0x7 => Unknown,
            0x8 => Any,
            _ => Self::default(),
        })
    }
}

/// Item metadata: the item's class, its granted abilities, and the shared
/// cooldown group id (if any).
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ItemMeta {
    class: ItemClass,
    abilities: &'static [WarcraftObjectId],
    cooldown_id: Option<WarcraftObjectId>,
}

impl ItemMeta {
    pub fn new(
        class: ItemClass,
        abilities: &'static [WarcraftObjectId],
        cooldown_id: Option<WarcraftObjectId>,
    ) -> Self {
        Self {
            class,
            abilities,
            cooldown_id,
        }
    }

    pub fn cooldown_id(&self) -> Option<WarcraftObjectId> {
        self.cooldown_id
    }

    pub fn abilities(&self) -> &'static [WarcraftObjectId] {
        self.abilities
    }

    pub fn class(&self) -> &ItemClass {
        &self.class
    }
}

// DDD roles: immutable, equality-by-value → Value Objects.
impl ddd::Layered for ItemClass {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for ItemClass {}

impl ddd::Layered for ItemMeta {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for ItemMeta {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_class_default_is_purchasable() {
        assert_eq!(ItemClass::default(), ItemClass::Purchasable);
    }

    #[test]
    fn item_class_try_from_known_strings() {
        assert_eq!(ItemClass::try_from("Artifact"), Ok(ItemClass::Artifact));
        assert_eq!(ItemClass::try_from("Permanent"), Ok(ItemClass::Permanent));
        assert_eq!(ItemClass::try_from("Charged"), Ok(ItemClass::Charged));
    }

    #[test]
    fn item_class_try_from_unknown_string_is_error() {
        assert!(ItemClass::try_from("NotAClass").is_err());
    }
}
