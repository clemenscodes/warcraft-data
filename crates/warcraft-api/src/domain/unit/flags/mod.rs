//! [`UnitFlags`]: the editor/campaign/special boolean flags of a unit.

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

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for UnitFlags {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for UnitFlags {}
