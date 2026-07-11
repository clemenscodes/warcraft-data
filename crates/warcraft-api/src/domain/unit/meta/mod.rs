//! [`UnitMeta`]: the kind-specific metadata aggregate for unit objects.

use crate::domain::identity::WarcraftObjectId;
use crate::domain::unit::combat::UnitCombat;
use crate::domain::unit::flags::UnitFlags;
use crate::domain::unit::hero::HeroAttributes;
use crate::domain::unit::kind::UnitKind;
use crate::domain::unit::mode::UnitMode;
use crate::domain::unit::production::UnitProduction;

#[derive(Default, Debug, Clone)]
pub struct UnitMeta {
    unit_kind: UnitKind,
    build_time: u32,
    /// In-game tech tier (`level` column of `unitbalance.slk`); lower = available
    /// earlier. Used to order the unit list by in-game availability.
    level: u32,
    /// Gold cost (`goldcost` column of `unitbalance.slk`). Used to order
    /// buildings by tier (the chain base's cost groups upgrade chains).
    gold_cost: u32,
    abilities: &'static [WarcraftObjectId],
    hero_abilities: &'static [WarcraftObjectId],
    researches: &'static [WarcraftObjectId],
    builds: &'static [WarcraftObjectId],
    trains: &'static [WarcraftObjectId],
    sell_items: &'static [WarcraftObjectId],
    sell_units: &'static [WarcraftObjectId],
    is_campaign: bool,
    is_in_editor: bool,
    is_hidden_in_editor: bool,
    is_special: bool,
    combat: UnitCombat,
    hero_attributes: Option<HeroAttributes>,
}

impl UnitMeta {
    pub const fn new(unit_kind: UnitKind, build_time: u32) -> Self {
        Self {
            unit_kind,
            build_time,
            level: 0,
            gold_cost: 0,
            abilities: &[],
            hero_abilities: &[],
            researches: &[],
            builds: &[],
            trains: &[],
            sell_items: &[],
            sell_units: &[],
            is_campaign: false,
            is_in_editor: true,
            is_hidden_in_editor: false,
            is_special: false,
            combat: UnitCombat::EMPTY,
            hero_attributes: None,
        }
    }

    pub const fn with_abilities(
        unit_kind: UnitKind,
        build_time: u32,
        abilities: &'static [WarcraftObjectId],
        hero_abilities: &'static [WarcraftObjectId],
    ) -> Self {
        Self {
            unit_kind,
            build_time,
            level: 0,
            gold_cost: 0,
            abilities,
            hero_abilities,
            researches: &[],
            builds: &[],
            trains: &[],
            sell_items: &[],
            sell_units: &[],
            is_campaign: false,
            is_in_editor: true,
            is_hidden_in_editor: false,
            is_special: false,
            combat: UnitCombat::EMPTY,
            hero_attributes: None,
        }
    }

    pub const fn with_full(
        unit_kind: UnitKind,
        build_time: u32,
        abilities: &'static [WarcraftObjectId],
        hero_abilities: &'static [WarcraftObjectId],
        is_campaign: bool,
        is_in_editor: bool,
        is_special: bool,
    ) -> Self {
        Self {
            unit_kind,
            build_time,
            level: 0,
            gold_cost: 0,
            abilities,
            hero_abilities,
            researches: &[],
            builds: &[],
            trains: &[],
            sell_items: &[],
            sell_units: &[],
            is_campaign,
            is_in_editor,
            is_hidden_in_editor: false,
            is_special,
            combat: UnitCombat::EMPTY,
            hero_attributes: None,
        }
    }

    pub const fn with_full_and_extras(
        unit_kind: UnitKind,
        build_time: u32,
        abilities: &'static [WarcraftObjectId],
        hero_abilities: &'static [WarcraftObjectId],
        production: UnitProduction,
        flags: UnitFlags,
    ) -> Self {
        Self {
            unit_kind,
            build_time,
            level: 0,
            gold_cost: 0,
            abilities,
            hero_abilities,
            researches: production.researches(),
            builds: production.builds(),
            trains: production.trains(),
            sell_items: production.sell_items(),
            sell_units: production.sell_units(),
            is_campaign: flags.is_campaign(),
            is_in_editor: flags.is_in_editor(),
            is_hidden_in_editor: flags.is_hidden_in_editor(),
            is_special: flags.is_special(),
            combat: UnitCombat::EMPTY,
            hero_attributes: None,
        }
    }

    pub const fn with_combat(mut self, combat: UnitCombat) -> Self {
        self.combat = combat;
        self
    }

    pub const fn with_hero_attributes(mut self, hero_attributes: HeroAttributes) -> Self {
        self.hero_attributes = Some(hero_attributes);
        self
    }

    pub const fn with_level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }

    pub const fn with_gold_cost(mut self, gold_cost: u32) -> Self {
        self.gold_cost = gold_cost;
        self
    }

    pub fn unit_kind(&self) -> UnitKind {
        self.unit_kind
    }

    pub fn build_time(&self) -> u32 {
        self.build_time
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn gold_cost(&self) -> u32 {
        self.gold_cost
    }

    pub fn abilities(&self) -> &'static [WarcraftObjectId] {
        self.abilities
    }

    pub fn hero_abilities(&self) -> &'static [WarcraftObjectId] {
        self.hero_abilities
    }

    pub fn builds(&self) -> &'static [WarcraftObjectId] {
        self.builds
    }

    pub fn trains(&self) -> &'static [WarcraftObjectId] {
        self.trains
    }

    pub fn is_campaign(&self) -> bool {
        self.is_campaign
    }

    pub fn is_in_editor(&self) -> bool {
        self.is_in_editor
    }

    pub fn is_hidden_in_editor(&self) -> bool {
        self.is_hidden_in_editor
    }

    pub fn is_special(&self) -> bool {
        self.is_special
    }

    pub fn researches(&self) -> &'static [WarcraftObjectId] {
        self.researches
    }

    pub fn sell_items(&self) -> &'static [WarcraftObjectId] {
        self.sell_items
    }

    pub fn sell_units(&self) -> &'static [WarcraftObjectId] {
        self.sell_units
    }

    pub fn is_melee_visible(&self) -> bool {
        self.is_in_editor && !self.is_campaign
    }

    pub fn combat(&self) -> &UnitCombat {
        &self.combat
    }

    pub fn hero_attributes(&self) -> Option<&HeroAttributes> {
        self.hero_attributes.as_ref()
    }

    /// The unit kind this meta behaves as on a command card. A "special" worker
    /// (militia-style unit that trains rather than gathers) is treated as a
    /// soldier so it gets the mobile-unit command set rather than the worker's
    /// build menu.
    pub fn effective_kind(&self) -> UnitKind {
        if self.is_special() && self.unit_kind() == UnitKind::Worker {
            return UnitKind::Soldier;
        }
        self.unit_kind()
    }

    /// Whether this unit should surface in a listing filtered for `mode`.
    ///
    /// `inEditor=1` in `unitui.slk` is Blizzard's flag for "show in the World
    /// Editor's unit picker". Tavern mercenaries with bindable abilities —
    /// Barbed Arachnathid (merc) `nanm` carrying Burrow, Watcher Ward `nwad`,
    /// Entangled Gold Mine `egol` — ship with `inEditor=0` because they aren't
    /// placed in the world editor, but they still need to surface in the hotkey
    /// editor's catalog. The downstream `has_visible_ability || has_production`
    /// check in `UnitCatalog::entries_for` still drops the placeholder rows
    /// (Barbed Arachnathid `nanb`, Crystal Arachnathid `nanc`, Warrior
    /// Arachnathid `nanw`) that have neither.
    pub fn passes_filter(&self, mode: UnitMode) -> bool {
        if self.is_hidden_in_editor() {
            return false;
        }
        match mode {
            UnitMode::Melee => !self.is_campaign(),
            UnitMode::Campaign => self.is_campaign(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_meta_new_defaults_to_in_editor_not_campaign() {
        let meta = UnitMeta::new(UnitKind::Soldier, 60);
        assert!(meta.is_in_editor());
        assert!(!meta.is_campaign());
    }

    #[test]
    fn unit_meta_is_melee_visible_only_when_in_editor_and_not_campaign() {
        let visible = UnitMeta::new(UnitKind::Soldier, 60);
        assert!(visible.is_melee_visible());
        let campaign = UnitMeta::with_full(UnitKind::Hero, 0, &[], &[], true, true, false);
        assert!(!campaign.is_melee_visible());
    }
}
