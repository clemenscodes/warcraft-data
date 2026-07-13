//! [`SystemHotkeysSlot`]: the rendered descriptor of a single ordered
//! system-hotkeys slot — the game section it binds plus the label the editor
//! shows for it.

use crate::WarcraftObjectId;

/// One slot in an ordered system-hotkeys group (an inventory slot, a hero
/// selection index, a control group). It pairs the game section id the slot
/// edits with the human label the editor renders for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SystemHotkeysSlot {
    section_id: WarcraftObjectId,
    label: &'static str,
}

impl SystemHotkeysSlot {
    /// Mint a slot descriptor. `pub(crate)` so only the crate's own ordered slot
    /// collections may produce one — a consumer receives them from
    /// [`InventorySlots`](crate::InventorySlots) and its siblings, never builds
    /// one from arbitrary parts.
    pub(crate) const fn new(section_id: WarcraftObjectId, label: &'static str) -> Self {
        Self { section_id, label }
    }

    pub fn section_id(self) -> WarcraftObjectId {
        self.section_id
    }

    pub fn label(self) -> &'static str {
        self.label
    }
}

// DDD role: a slot descriptor is an immutable Value Object.
impl ddd::Layered for SystemHotkeysSlot {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for SystemHotkeysSlot {}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_value_object<Type>()
    where
        Type: ddd::ValueObject,
    {
    }

    #[test]
    fn system_hotkeys_slot_is_a_value_object() {
        assert_value_object::<SystemHotkeysSlot>();
    }
}
