//! [`HeroSelectionSlots`]: the three hero-selection slots, in order, each as its
//! own distinct marker type so the set is exhaustive and unforgeable.

use crate::WarcraftObjectId;
use crate::domain::keybind::slots::system_hotkeys_slot::SystemHotkeysSlot;

// The three hero-selection slots, each as its own distinct type. Because the
// types differ, no value of one can ever stand in for another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HeroSelectionSlot1;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HeroSelectionSlot2;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HeroSelectionSlot3;

impl From<HeroSelectionSlot1> for SystemHotkeysSlot {
    fn from(_: HeroSelectionSlot1) -> Self {
        let section_id = WarcraftObjectId::new("her1");
        Self::new(section_id, "Hero 1")
    }
}
impl From<HeroSelectionSlot2> for SystemHotkeysSlot {
    fn from(_: HeroSelectionSlot2) -> Self {
        let section_id = WarcraftObjectId::new("her2");
        Self::new(section_id, "Hero 2")
    }
}
impl From<HeroSelectionSlot3> for SystemHotkeysSlot {
    fn from(_: HeroSelectionSlot3) -> Self {
        let section_id = WarcraftObjectId::new("her3");
        Self::new(section_id, "Hero 3")
    }
}

/// The set of all hero-selection slots, in editor order. Its **only** inhabitant
/// is the three distinct slots, each exactly once: every position is a different
/// type, so a duplicate, an omission, or a fourth entry does not type-check. The
/// fields are private, the one value is [`HeroSelectionSlots::ALL`], and a
/// consumer only ever iterates it with [`HeroSelectionSlots::iter`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HeroSelectionSlots(HeroSelectionSlot1, HeroSelectionSlot2, HeroSelectionSlot3);

impl HeroSelectionSlots {
    pub const ALL: Self = Self(HeroSelectionSlot1, HeroSelectionSlot2, HeroSelectionSlot3);

    /// The three hero-selection slots, in editor order, as plain
    /// [`SystemHotkeysSlot`] descriptors for a consumer that just wants to loop.
    pub fn iter(self) -> impl Iterator<Item = SystemHotkeysSlot> {
        let slots = [
            SystemHotkeysSlot::from(self.0),
            SystemHotkeysSlot::from(self.1),
            SystemHotkeysSlot::from(self.2),
        ];
        slots.into_iter()
    }
}

// DDD role: the exhaustive hero-selection slot set is an immutable Value Object.
impl ddd::Layered for HeroSelectionSlots {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for HeroSelectionSlots {}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_value_object<Type>()
    where
        Type: ddd::ValueObject,
    {
    }

    #[test]
    fn hero_selection_slots_is_a_value_object() {
        assert_value_object::<HeroSelectionSlots>();
    }

    #[test]
    fn iterates_three_slots_in_order() {
        let slots: Vec<SystemHotkeysSlot> = HeroSelectionSlots::ALL.iter().collect();
        let section_ids: Vec<&'static str> =
            slots.iter().map(|slot| slot.section_id().value()).collect();
        let labels: Vec<&'static str> = slots.iter().map(|slot| slot.label()).collect();
        assert_eq!(section_ids, ["her1", "her2", "her3"]);
        assert_eq!(labels, ["Hero 1", "Hero 2", "Hero 3"]);
    }
}
