//! [`WarcraftObject`]: the object aggregate root. Any addressable Warcraft III
//! object — a unit, ability, upgrade, item, or command — identified by its
//! [`WarcraftObjectId`], owning its kind, display text, race, and kind-specific
//! metadata.

use crate::domain::grid::GridCoordinate;
use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::kind::WarcraftObjectKind;
use crate::domain::object::meta::WarcraftObjectMeta;
use crate::domain::race::Race;

const ICON_PATH_BLACKLIST: &[&str] = &["commandbuttons/btnselectheroon.blp"];

#[derive(Default, Debug, Clone)]
pub struct WarcraftObject {
    id: WarcraftObjectId,
    names: &'static [&'static str],
    icons: &'static [&'static str],
    kind: WarcraftObjectKind,
    race: Option<Race>,
    meta: WarcraftObjectMeta,
    tip_levels: &'static [&'static str],
    ubertip_levels: &'static [&'static str],
    un_tip: Option<&'static str>,
    un_ubertip: Option<&'static str>,
    default_button_position: Option<GridCoordinate>,
    default_research_button_position: Option<GridCoordinate>,
}

impl WarcraftObject {
    pub fn new(
        id: WarcraftObjectId,
        names: &'static [&'static str],
        icons: &'static [&'static str],
        kind: WarcraftObjectKind,
        race: Option<Race>,
        meta: WarcraftObjectMeta,
    ) -> Self {
        Self {
            id,
            names,
            icons,
            kind,
            race,
            meta,
            tip_levels: &[],
            ubertip_levels: &[],
            un_tip: None,
            un_ubertip: None,
            default_button_position: None,
            default_research_button_position: None,
        }
    }

    pub fn with_text(
        id: WarcraftObjectId,
        names: &'static [&'static str],
        icons: &'static [&'static str],
        kind: WarcraftObjectKind,
        race: Option<Race>,
        meta: WarcraftObjectMeta,
        text: crate::domain::object::text::WarcraftObjectText,
    ) -> Self {
        Self {
            id,
            names,
            icons,
            kind,
            race,
            meta,
            tip_levels: text.tip_levels,
            ubertip_levels: text.ubertip_levels,
            un_tip: text.un_tip,
            un_ubertip: text.un_ubertip,
            default_button_position: None,
            default_research_button_position: None,
        }
    }

    pub fn with_default_position(mut self, position: Option<GridCoordinate>) -> Self {
        self.default_button_position = position;
        self
    }

    pub fn with_default_research_position(mut self, position: Option<GridCoordinate>) -> Self {
        self.default_research_button_position = position;
        self
    }

    pub fn id(&self) -> WarcraftObjectId {
        self.id
    }

    pub fn names(&self) -> &'static [&'static str] {
        self.names
    }

    pub fn icons(&self) -> &'static [&'static str] {
        self.icons
    }

    pub fn kind(&self) -> WarcraftObjectKind {
        self.kind
    }

    pub fn race(&self) -> Option<Race> {
        self.race
    }

    pub fn meta(&self) -> &WarcraftObjectMeta {
        &self.meta
    }

    pub fn tip(&self) -> Option<&'static str> {
        if let Some(first) = self.tip_levels.first() {
            return Some(*first);
        }
        if let WarcraftObjectMeta::Command(command_meta) = &self.meta {
            return command_meta.tip();
        }
        None
    }

    pub fn ubertip(&self) -> Option<&'static str> {
        if let Some(first) = self.ubertip_levels.first() {
            return Some(*first);
        }
        match &self.meta {
            WarcraftObjectMeta::Ability(ability_meta) => ability_meta.ubertip(),
            WarcraftObjectMeta::Command(command_meta) => command_meta.ubertip(),
            _ => None,
        }
    }

    pub fn tip_levels(&self) -> &'static [&'static str] {
        self.tip_levels
    }

    pub fn ubertip_levels(&self) -> &'static [&'static str] {
        self.ubertip_levels
    }

    pub fn research_ubertip(&self) -> Option<&'static str> {
        if let WarcraftObjectMeta::Ability(ability_meta) = &self.meta {
            return ability_meta.research_ubertip();
        }
        None
    }

    pub fn un_tip(&self) -> Option<&'static str> {
        self.un_tip
    }

    pub fn un_ubertip(&self) -> Option<&'static str> {
        self.un_ubertip
    }

    pub fn default_button_position(&self) -> Option<GridCoordinate> {
        if self.default_button_position.is_some() {
            return self.default_button_position;
        }
        match &self.meta {
            WarcraftObjectMeta::Ability(ability_meta) => ability_meta.default_button_position(),
            WarcraftObjectMeta::Command(command_meta) => command_meta.default_button_position(),
            _ => None,
        }
    }

    pub fn default_research_button_position(&self) -> Option<GridCoordinate> {
        if self.default_research_button_position.is_some() {
            return self.default_research_button_position;
        }
        match &self.meta {
            WarcraftObjectMeta::Ability(ability_meta) => {
                ability_meta.default_research_button_position()
            }
            _ => None,
        }
    }

    pub fn is_ultimate_ability(&self) -> bool {
        match self.meta() {
            WarcraftObjectMeta::Ability(ability_meta) => ability_meta.is_ultimate(),
            _ => false,
        }
    }

    pub fn cooldowns(&self) -> Option<[u32; 4]> {
        match self.meta() {
            WarcraftObjectMeta::Ability(ability_meta) => Some(ability_meta.cooldowns()),
            _ => None,
        }
    }

    pub fn has_displayable_icon(&self) -> bool {
        self.icons().iter().any(|icon_path| {
            if icon_path.trim().is_empty() {
                return false;
            }
            let normalized = icon_path.trim().to_ascii_lowercase();
            !ICON_PATH_BLACKLIST.contains(&normalized.as_str())
        })
    }

    pub fn is_passive_ability(&self) -> bool {
        self.icons()
            .first()
            .map(|icon_path| {
                icon_path
                    .to_ascii_lowercase()
                    .starts_with("passivebuttons/")
            })
            .unwrap_or(false)
    }

    pub fn ability_code(&self) -> Option<WarcraftObjectId> {
        match &self.meta {
            WarcraftObjectMeta::Ability(ability_meta) => ability_meta.code(),
            _ => None,
        }
    }

    pub fn ability_morph_target_id(&self) -> Option<WarcraftObjectId> {
        match &self.meta {
            WarcraftObjectMeta::Ability(ability_meta) => ability_meta.morph_target_unit().copied(),
            _ => None,
        }
    }

    /// The number of research tiers this object has, but only when it is an
    /// upgrade. Multi-level upgrades store one hotkey token per tier
    /// (`Hotkey=F,F,F`), so the editor needs this to size that list. Leveled
    /// abilities (hero spells, auras) are deliberately excluded: their
    /// command-card button is shared across levels and binds a single hotkey, so
    /// they must not be replicated. Non-upgrades return `None`.
    pub fn upgrade_max_level(&self) -> Option<usize> {
        match &self.meta {
            WarcraftObjectMeta::Upgrade(upgrade_meta) => Some(upgrade_meta.max_level()),
            _ => None,
        }
    }

    pub fn ability_off_icon(&self) -> Option<&'static str> {
        match &self.meta {
            WarcraftObjectMeta::Ability(ability_meta) => ability_meta.off_icon(),
            _ => None,
        }
    }
}

// DDD role: the object aggregate — identified by, and distinguished by, its id.
impl ddd::Layered for WarcraftObject {
    type Layer = ddd::DomainLayer;
}
impl ddd::Entity for WarcraftObject {
    type Identity = WarcraftObjectId;

    fn identity(&self) -> &Self::Identity {
        &self.id
    }
}
impl ddd::AggregateRoot for WarcraftObject {}
