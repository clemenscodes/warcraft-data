//! [`DefaultHotkeys`]: the hotkeys the game itself assigns to one section.

/// The hotkeys one section of a game text file assigns, read verbatim from
/// the archive: either Blizzard's shipped `war3.w3mod:customkeys.txt` (the
/// game's own default keybinds) or a localized `*strings.txt`. An empty value
/// (`Hotkey=`) is kept as `Some("")`: the game leaves that button deliberately
/// unbound. `None` means the section does not mention the field at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct DefaultHotkeys {
    section_id: &'static str,
    hotkey: Option<&'static str>,
    unhotkey: Option<&'static str>,
    research_hotkey: Option<&'static str>,
}

impl DefaultHotkeys {
    pub(crate) const fn new(
        section_id: &'static str,
        hotkey: Option<&'static str>,
        unhotkey: Option<&'static str>,
        research_hotkey: Option<&'static str>,
    ) -> Self {
        Self {
            section_id,
            hotkey,
            unhotkey,
            research_hotkey,
        }
    }

    pub fn section_id(&self) -> &'static str {
        self.section_id
    }

    pub fn hotkey(&self) -> Option<&'static str> {
        self.hotkey
    }

    pub fn unhotkey(&self) -> Option<&'static str> {
        self.unhotkey
    }

    pub fn research_hotkey(&self) -> Option<&'static str> {
        self.research_hotkey
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for DefaultHotkeys {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for DefaultHotkeys {}
