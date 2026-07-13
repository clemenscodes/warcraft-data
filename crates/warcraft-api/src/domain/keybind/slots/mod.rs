//! Slots concern: the ordered slot collections for the system-hotkeys editor —
//! inventory, hero selection and control groups — plus the shared slot
//! descriptor they render as. Each collection mirrors the [`AllRaces`] shape:
//! distinct marker types, a private-field tuple struct, a single `ALL`, and an
//! `iter` that yields plain [`SystemHotkeysSlot`] descriptors.
//!
//! [`AllRaces`]: crate::AllRaces

pub(crate) mod control_group_slots;
pub(crate) mod hero_selection_slots;
pub(crate) mod inventory_slots;
pub(crate) mod system_hotkeys_slot;

pub use control_group_slots::ControlGroupSlots;
pub use hero_selection_slots::HeroSelectionSlots;
pub use inventory_slots::InventorySlots;
pub use system_hotkeys_slot::SystemHotkeysSlot;
