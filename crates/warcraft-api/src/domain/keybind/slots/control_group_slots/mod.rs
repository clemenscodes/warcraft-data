//! [`ControlGroupSlots`]: the ten control-group slots, in order, each as its own
//! distinct marker type so the set is exhaustive and unforgeable.

use crate::WarcraftObjectId;
use crate::domain::keybind::slots::system_hotkeys_slot::SystemHotkeysSlot;

// The ten control-group slots, each as its own distinct type. Because the types
// differ, no value of one can ever stand in for another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ControlGroupSlot1;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ControlGroupSlot2;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ControlGroupSlot3;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ControlGroupSlot4;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ControlGroupSlot5;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ControlGroupSlot6;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ControlGroupSlot7;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ControlGroupSlot8;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ControlGroupSlot9;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ControlGroupSlot10;

impl From<ControlGroupSlot1> for SystemHotkeysSlot {
    fn from(_: ControlGroupSlot1) -> Self {
        let section_id = WarcraftObjectId::new("Ctr1");
        Self::new(section_id, "1")
    }
}
impl From<ControlGroupSlot2> for SystemHotkeysSlot {
    fn from(_: ControlGroupSlot2) -> Self {
        let section_id = WarcraftObjectId::new("Ctr2");
        Self::new(section_id, "2")
    }
}
impl From<ControlGroupSlot3> for SystemHotkeysSlot {
    fn from(_: ControlGroupSlot3) -> Self {
        let section_id = WarcraftObjectId::new("Ctr3");
        Self::new(section_id, "3")
    }
}
impl From<ControlGroupSlot4> for SystemHotkeysSlot {
    fn from(_: ControlGroupSlot4) -> Self {
        let section_id = WarcraftObjectId::new("Ctr4");
        Self::new(section_id, "4")
    }
}
impl From<ControlGroupSlot5> for SystemHotkeysSlot {
    fn from(_: ControlGroupSlot5) -> Self {
        let section_id = WarcraftObjectId::new("Ctr5");
        Self::new(section_id, "5")
    }
}
impl From<ControlGroupSlot6> for SystemHotkeysSlot {
    fn from(_: ControlGroupSlot6) -> Self {
        let section_id = WarcraftObjectId::new("Ctr6");
        Self::new(section_id, "6")
    }
}
impl From<ControlGroupSlot7> for SystemHotkeysSlot {
    fn from(_: ControlGroupSlot7) -> Self {
        let section_id = WarcraftObjectId::new("Ctr7");
        Self::new(section_id, "7")
    }
}
impl From<ControlGroupSlot8> for SystemHotkeysSlot {
    fn from(_: ControlGroupSlot8) -> Self {
        let section_id = WarcraftObjectId::new("Ctr8");
        Self::new(section_id, "8")
    }
}
impl From<ControlGroupSlot9> for SystemHotkeysSlot {
    fn from(_: ControlGroupSlot9) -> Self {
        let section_id = WarcraftObjectId::new("Ctr9");
        Self::new(section_id, "9")
    }
}
impl From<ControlGroupSlot10> for SystemHotkeysSlot {
    fn from(_: ControlGroupSlot10) -> Self {
        let section_id = WarcraftObjectId::new("Ctr0");
        Self::new(section_id, "10")
    }
}

/// The set of all control-group slots, in editor order (groups 1 through 9 then
/// group 10, whose section id is `Ctr0`). Its **only** inhabitant is the ten
/// distinct slots, each exactly once: every position is a different type, so a
/// duplicate, an omission, or an eleventh entry does not type-check. The fields
/// are private, the one value is [`ControlGroupSlots::ALL`], and a consumer only
/// ever iterates it with [`ControlGroupSlots::iter`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ControlGroupSlots(
    ControlGroupSlot1,
    ControlGroupSlot2,
    ControlGroupSlot3,
    ControlGroupSlot4,
    ControlGroupSlot5,
    ControlGroupSlot6,
    ControlGroupSlot7,
    ControlGroupSlot8,
    ControlGroupSlot9,
    ControlGroupSlot10,
);

impl ControlGroupSlots {
    pub const ALL: Self = Self(
        ControlGroupSlot1,
        ControlGroupSlot2,
        ControlGroupSlot3,
        ControlGroupSlot4,
        ControlGroupSlot5,
        ControlGroupSlot6,
        ControlGroupSlot7,
        ControlGroupSlot8,
        ControlGroupSlot9,
        ControlGroupSlot10,
    );

    /// The ten control-group slots, in editor order, as plain
    /// [`SystemHotkeysSlot`] descriptors for a consumer that just wants to loop.
    pub fn iter(self) -> impl Iterator<Item = SystemHotkeysSlot> {
        let slots = [
            SystemHotkeysSlot::from(self.0),
            SystemHotkeysSlot::from(self.1),
            SystemHotkeysSlot::from(self.2),
            SystemHotkeysSlot::from(self.3),
            SystemHotkeysSlot::from(self.4),
            SystemHotkeysSlot::from(self.5),
            SystemHotkeysSlot::from(self.6),
            SystemHotkeysSlot::from(self.7),
            SystemHotkeysSlot::from(self.8),
            SystemHotkeysSlot::from(self.9),
        ];
        slots.into_iter()
    }
}

// DDD role: the exhaustive control-group slot set is an immutable Value Object.
impl ddd::Layered for ControlGroupSlots {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for ControlGroupSlots {}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_value_object<Type>()
    where
        Type: ddd::ValueObject,
    {
    }

    #[test]
    fn control_group_slots_is_a_value_object() {
        assert_value_object::<ControlGroupSlots>();
    }

    #[test]
    fn iterates_ten_slots_in_order() {
        let slots: Vec<SystemHotkeysSlot> = ControlGroupSlots::ALL.iter().collect();
        let section_ids: Vec<&'static str> =
            slots.iter().map(|slot| slot.section_id().value()).collect();
        let labels: Vec<&'static str> = slots.iter().map(|slot| slot.label()).collect();
        assert_eq!(
            section_ids,
            [
                "Ctr1", "Ctr2", "Ctr3", "Ctr4", "Ctr5", "Ctr6", "Ctr7", "Ctr8", "Ctr9", "Ctr0"
            ]
        );
        assert_eq!(labels, ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]);
    }
}
