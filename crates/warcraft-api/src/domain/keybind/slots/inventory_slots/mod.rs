//! [`InventorySlots`]: the six inventory slots, in order, each as its own
//! distinct marker type so the set is exhaustive and unforgeable.

use crate::WarcraftObjectId;
use crate::domain::keybind::slots::system_hotkeys_slot::SystemHotkeysSlot;

// The six inventory slots, each as its own distinct type. Because the types
// differ, no value of one can ever stand in for another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InventorySlot1;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InventorySlot2;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InventorySlot3;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InventorySlot4;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InventorySlot5;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InventorySlot6;

impl From<InventorySlot1> for SystemHotkeysSlot {
    fn from(_: InventorySlot1) -> Self {
        let section_id = WarcraftObjectId::new("itm1");
        Self::new(section_id, "Slot 1")
    }
}
impl From<InventorySlot2> for SystemHotkeysSlot {
    fn from(_: InventorySlot2) -> Self {
        let section_id = WarcraftObjectId::new("itm2");
        Self::new(section_id, "Slot 2")
    }
}
impl From<InventorySlot3> for SystemHotkeysSlot {
    fn from(_: InventorySlot3) -> Self {
        let section_id = WarcraftObjectId::new("itm3");
        Self::new(section_id, "Slot 3")
    }
}
impl From<InventorySlot4> for SystemHotkeysSlot {
    fn from(_: InventorySlot4) -> Self {
        let section_id = WarcraftObjectId::new("itm4");
        Self::new(section_id, "Slot 4")
    }
}
impl From<InventorySlot5> for SystemHotkeysSlot {
    fn from(_: InventorySlot5) -> Self {
        let section_id = WarcraftObjectId::new("itm5");
        Self::new(section_id, "Slot 5")
    }
}
impl From<InventorySlot6> for SystemHotkeysSlot {
    fn from(_: InventorySlot6) -> Self {
        let section_id = WarcraftObjectId::new("itm6");
        Self::new(section_id, "Slot 6")
    }
}

/// The set of all inventory slots, in editor order. Its **only** inhabitant is
/// the six distinct slots, each exactly once: every position is a different
/// type, so a duplicate, an omission, or a seventh entry does not type-check.
/// The fields are private, the one value is [`InventorySlots::ALL`], and a
/// consumer only ever iterates it with [`InventorySlots::iter`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InventorySlots(
    InventorySlot1,
    InventorySlot2,
    InventorySlot3,
    InventorySlot4,
    InventorySlot5,
    InventorySlot6,
);

impl InventorySlots {
    pub const ALL: Self = Self(
        InventorySlot1,
        InventorySlot2,
        InventorySlot3,
        InventorySlot4,
        InventorySlot5,
        InventorySlot6,
    );

    /// Columns in the inventory command-card grid the slots lay out onto.
    pub const COLUMNS: usize = 2;
    /// Rows in the inventory command-card grid the slots lay out onto.
    pub const ROWS: usize = 3;

    /// The six inventory slots, in editor order, as plain [`SystemHotkeysSlot`]
    /// descriptors for a consumer that just wants to loop.
    pub fn iter(self) -> impl Iterator<Item = SystemHotkeysSlot> {
        let slots = [
            SystemHotkeysSlot::from(self.0),
            SystemHotkeysSlot::from(self.1),
            SystemHotkeysSlot::from(self.2),
            SystemHotkeysSlot::from(self.3),
            SystemHotkeysSlot::from(self.4),
            SystemHotkeysSlot::from(self.5),
        ];
        slots.into_iter()
    }
}

// DDD role: the exhaustive inventory slot set is an immutable Value Object.
impl ddd::Layered for InventorySlots {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for InventorySlots {}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_value_object<Type>()
    where
        Type: ddd::ValueObject,
    {
    }

    #[test]
    fn inventory_slots_is_a_value_object() {
        assert_value_object::<InventorySlots>();
    }

    #[test]
    fn iterates_six_slots_in_order() {
        let slots: Vec<SystemHotkeysSlot> = InventorySlots::ALL.iter().collect();
        let section_ids: Vec<&'static str> =
            slots.iter().map(|slot| slot.section_id().value()).collect();
        let labels: Vec<&'static str> = slots.iter().map(|slot| slot.label()).collect();
        assert_eq!(
            section_ids,
            ["itm1", "itm2", "itm3", "itm4", "itm5", "itm6"]
        );
        assert_eq!(
            labels,
            ["Slot 1", "Slot 2", "Slot 3", "Slot 4", "Slot 5", "Slot 6"]
        );
    }

    #[test]
    fn exposes_grid_geometry() {
        assert_eq!(InventorySlots::COLUMNS, 2);
        assert_eq!(InventorySlots::ROWS, 3);
    }
}
