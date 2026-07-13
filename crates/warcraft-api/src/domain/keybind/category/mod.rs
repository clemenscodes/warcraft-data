use std::fmt;

use crate::{
    ControlGroupSlots, HeroSelectionSlots, InventorySlots, SystemKeybind, SystemKeybindClass,
    WarcraftObjectId,
};

use crate::WARCRAFT_SYSTEM_KEYBINDS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemHotkeysCategory {
    Inventory,
    HeroSelection,
    ControlGroups,
    GeneralCommands,
    Menu,
    Camera,
    Observer,
    Replay,
}

impl SystemHotkeysCategory {
    pub const ALL: [SystemHotkeysCategory; 8] = [
        SystemHotkeysCategory::Inventory,
        SystemHotkeysCategory::HeroSelection,
        SystemHotkeysCategory::ControlGroups,
        SystemHotkeysCategory::GeneralCommands,
        SystemHotkeysCategory::Menu,
        SystemHotkeysCategory::Camera,
        SystemHotkeysCategory::Observer,
        SystemHotkeysCategory::Replay,
    ];

    /// The per-category intro text the editor shows above the category's slots,
    /// or `None` for categories that render without a caption.
    pub fn caption(self) -> Option<&'static str> {
        match self {
            SystemHotkeysCategory::Inventory => {
                Some("Drag a slot onto another to swap their keys.")
            }
            SystemHotkeysCategory::HeroSelection => {
                Some("Hotkeys for selecting your heroes by index.")
            }
            SystemHotkeysCategory::ControlGroups => Some("Hotkeys for control groups 1–10."),
            SystemHotkeysCategory::GeneralCommands
            | SystemHotkeysCategory::Menu
            | SystemHotkeysCategory::Camera
            | SystemHotkeysCategory::Observer
            | SystemHotkeysCategory::Replay => None,
        }
    }

    pub fn entries(self) -> Vec<&'static SystemKeybind> {
        match self {
            SystemHotkeysCategory::Inventory => {
                let section_ids = InventorySlots::ALL.iter().map(|slot| slot.section_id());
                Self::collect_in_order(section_ids)
            }
            SystemHotkeysCategory::HeroSelection => {
                let section_ids = HeroSelectionSlots::ALL.iter().map(|slot| slot.section_id());
                Self::collect_in_order(section_ids)
            }
            SystemHotkeysCategory::ControlGroups => {
                let section_ids = ControlGroupSlots::ALL.iter().map(|slot| slot.section_id());
                Self::collect_in_order(section_ids)
            }
            SystemHotkeysCategory::GeneralCommands => Self::collect_general_commands(),
            SystemHotkeysCategory::Menu => Self::collect_by_class(SystemKeybindClass::Menu),
            SystemHotkeysCategory::Camera => Self::collect_by_class(SystemKeybindClass::Camera),
            SystemHotkeysCategory::Observer => Self::collect_by_class(SystemKeybindClass::Observer),
            SystemHotkeysCategory::Replay => Self::collect_by_class(SystemKeybindClass::Replay),
        }
    }

    fn collect_in_order(
        section_ids: impl IntoIterator<Item = WarcraftObjectId>,
    ) -> Vec<&'static SystemKeybind> {
        let wanted_ids: Vec<WarcraftObjectId> = section_ids.into_iter().collect();
        let mut ordered: Vec<&'static SystemKeybind> = Vec::with_capacity(wanted_ids.len());
        for wanted_id in wanted_ids {
            for entry in WARCRAFT_SYSTEM_KEYBINDS.iter() {
                if entry.section_id() == wanted_id {
                    ordered.push(entry);
                    break;
                }
            }
        }
        ordered
    }

    fn collect_by_class(class: SystemKeybindClass) -> Vec<&'static SystemKeybind> {
        WARCRAFT_SYSTEM_KEYBINDS
            .iter()
            .filter(|entry| entry.class() == class)
            .collect()
    }

    fn collect_general_commands() -> Vec<&'static SystemKeybind> {
        let inventory_ids: Vec<WarcraftObjectId> = InventorySlots::ALL
            .iter()
            .map(|slot| slot.section_id())
            .collect();
        let hero_selection_ids: Vec<WarcraftObjectId> = HeroSelectionSlots::ALL
            .iter()
            .map(|slot| slot.section_id())
            .collect();
        WARCRAFT_SYSTEM_KEYBINDS
            .iter()
            .filter(|entry| {
                if entry.class() != SystemKeybindClass::Game {
                    return false;
                }
                let id = entry.section_id();
                !inventory_ids.contains(&id) && !hero_selection_ids.contains(&id)
            })
            .collect()
    }
}

impl fmt::Display for SystemHotkeysCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            SystemHotkeysCategory::Inventory => "Inventory",
            SystemHotkeysCategory::HeroSelection => "Hero Selection",
            SystemHotkeysCategory::ControlGroups => "Control Groups",
            SystemHotkeysCategory::GeneralCommands => "General Commands",
            SystemHotkeysCategory::Menu => "Menu",
            SystemHotkeysCategory::Camera => "Camera",
            SystemHotkeysCategory::Observer => "Observer Mode",
            SystemHotkeysCategory::Replay => "Replay",
        };
        formatter.write_str(label)
    }
}

impl ddd::Layered for SystemHotkeysCategory {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for SystemHotkeysCategory {}
