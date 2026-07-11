//! [`AbilityMeta`]: kind-specific metadata for ability objects.

use crate::domain::grid::GridCoordinate;
use crate::domain::identity::WarcraftObjectId;
use crate::domain::quantity::Chance;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AbilityMeta {
    max_level: usize,
    is_ultimate: bool,
    cooldowns: [u32; 4],
    /// Per-level chance to evade an attack, one slot per ability level. Non-zero
    /// only for evasion abilities (Evasion `AEev`, Drunken Brawler `ANdb`);
    /// every other ability leaves this all-zero. Sourced from the real
    /// `abilitydata.slk` data field, not the tooltip text.
    evasion_chances: [Chance; 4],
    default_button_position: Option<GridCoordinate>,
    default_research_button_position: Option<GridCoordinate>,
    ubertip: Option<&'static str>,
    research_ubertip: Option<&'static str>,
    code: Option<WarcraftObjectId>,
    morph_target_unit: Option<WarcraftObjectId>,
    off_button_position: Option<GridCoordinate>,
    off_tip: Option<&'static str>,
    off_ubertip: Option<&'static str>,
    off_icon: Option<&'static str>,
}

impl AbilityMeta {
    pub const fn new(max_level: usize, is_ultimate: bool, cooldowns: [u32; 4]) -> Self {
        Self {
            max_level,
            is_ultimate,
            cooldowns,
            evasion_chances: [Chance::from_permille(0); 4],
            default_button_position: None,
            default_research_button_position: None,
            ubertip: None,
            research_ubertip: None,
            code: None,
            morph_target_unit: None,
            off_button_position: None,
            off_tip: None,
            off_ubertip: None,
            off_icon: None,
        }
    }

    pub const fn with_defaults(
        max_level: usize,
        is_ultimate: bool,
        cooldowns: [u32; 4],
        default_button_position: Option<GridCoordinate>,
        default_research_button_position: Option<GridCoordinate>,
    ) -> Self {
        Self {
            max_level,
            is_ultimate,
            cooldowns,
            evasion_chances: [Chance::from_permille(0); 4],
            default_button_position,
            default_research_button_position,
            ubertip: None,
            research_ubertip: None,
            code: None,
            morph_target_unit: None,
            off_button_position: None,
            off_tip: None,
            off_ubertip: None,
            off_icon: None,
        }
    }

    pub const fn with_ubertips(
        max_level: usize,
        is_ultimate: bool,
        cooldowns: [u32; 4],
        default_button_position: Option<GridCoordinate>,
        default_research_button_position: Option<GridCoordinate>,
        ubertip: Option<&'static str>,
        research_ubertip: Option<&'static str>,
    ) -> Self {
        Self {
            max_level,
            is_ultimate,
            cooldowns,
            evasion_chances: [Chance::from_permille(0); 4],
            default_button_position,
            default_research_button_position,
            ubertip,
            research_ubertip,
            code: None,
            morph_target_unit: None,
            off_button_position: None,
            off_tip: None,
            off_ubertip: None,
            off_icon: None,
        }
    }

    pub const fn with_code(mut self, code: Option<WarcraftObjectId>) -> Self {
        self.code = code;
        self
    }

    pub const fn with_morph_target(mut self, target: Option<WarcraftObjectId>) -> Self {
        self.morph_target_unit = target;
        self
    }

    pub const fn with_evasion_chances(mut self, evasion_chances: [Chance; 4]) -> Self {
        self.evasion_chances = evasion_chances;
        self
    }

    pub const fn with_off_state(
        mut self,
        off_button_position: Option<GridCoordinate>,
        off_tip: Option<&'static str>,
        off_ubertip: Option<&'static str>,
        off_icon: Option<&'static str>,
    ) -> Self {
        self.off_button_position = off_button_position;
        self.off_tip = off_tip;
        self.off_ubertip = off_ubertip;
        self.off_icon = off_icon;
        self
    }

    pub fn ubertip(&self) -> Option<&'static str> {
        self.ubertip
    }

    pub fn research_ubertip(&self) -> Option<&'static str> {
        self.research_ubertip
    }

    /// Game-mechanic class as listed in `units/abilitydata.slk`'s `code`
    /// column. Independent of the per-unit alias — e.g. multiple aliases
    /// can resolve to `code = "Apit"` (Purchase Item / shop button).
    pub fn code(&self) -> Option<WarcraftObjectId> {
        self.code
    }

    /// For one-way morph abilities (Avenger Form, Crow Form, etc.) the
    /// unit id this ability transforms its caster into. Sourced from the
    /// `UnitID1` column of `abilitydata.slk`.
    pub fn morph_target_unit(&self) -> Option<&WarcraftObjectId> {
        self.morph_target_unit.as_ref()
    }

    /// Off-state button position for toggleable abilities (e.g. Defend on
    /// the Footman). Some abilities place their "deactivate" cell at a
    /// different grid slot when active. Sourced from `UnButtonpos=` in
    /// `abilityfunc.txt`.
    pub fn off_button_position(&self) -> Option<GridCoordinate> {
        self.off_button_position
    }

    /// Off-state short tooltip — the label shown while the ability is
    /// active (e.g. "Stop Defending" while Defend is on). Sourced from
    /// `UnTip=` in `abilityfunc.txt`.
    pub fn off_tip(&self) -> Option<&'static str> {
        self.off_tip
    }

    /// Off-state long description — `UnUbertip=` in `abilityfunc.txt`.
    pub fn off_ubertip(&self) -> Option<&'static str> {
        self.off_ubertip
    }

    /// Off-state icon path (`UnArt=` in `abilityfunc.txt`). Different art
    /// from the on-state icon for toggle abilities like Defend, whose
    /// active state shows a distinct "Stop Defending" art.
    pub fn off_icon(&self) -> Option<&'static str> {
        self.off_icon
    }

    /// Returns true if the ability has any off-state data in the database.
    /// One-shot abilities (e.g. Healing Wave) have all four off-state fields
    /// set to None and must not receive a materialized unbutton_position.
    pub fn has_off_state(&self) -> bool {
        let position_set = self.off_button_position.is_some();
        let tip_set = self.off_tip.is_some();
        let ubertip_set = self.off_ubertip.is_some();
        let icon_set = self.off_icon.is_some();
        position_set || tip_set || ubertip_set || icon_set
    }

    pub fn default_button_position(&self) -> Option<GridCoordinate> {
        self.default_button_position
    }

    pub fn default_research_button_position(&self) -> Option<GridCoordinate> {
        self.default_research_button_position
    }

    pub fn max_level(&self) -> usize {
        self.max_level
    }

    pub fn is_ultimate(&self) -> bool {
        self.is_ultimate
    }

    pub fn cooldown_for_level(&self, level: usize) -> Option<u32> {
        if level == 0 || level > self.max_level {
            None
        } else {
            Some(self.cooldowns[level - 1])
        }
    }

    pub fn base_cooldown(&self) -> u32 {
        self.cooldowns[0]
    }

    pub fn cooldowns(&self) -> [u32; 4] {
        self.cooldowns
    }

    /// Per-level chance to evade an attack. All-zero for any ability that is not
    /// an evasion ability.
    pub fn evasion_chances(&self) -> [Chance; 4] {
        self.evasion_chances
    }

    /// Chance to evade at a given ability level (1-based), or `None` when the
    /// level is out of range. Levels beyond `max_level` are not real.
    pub fn evasion_chance_for_level(&self, level: usize) -> Option<Chance> {
        if level == 0 || level > self.max_level {
            None
        } else {
            Some(self.evasion_chances[level - 1])
        }
    }

    /// True if the ability grants any evasion at any level.
    pub fn has_evasion(&self) -> bool {
        self.evasion_chances.iter().any(|chance| chance.is_some())
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for AbilityMeta {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for AbilityMeta {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::grid::{ColumnIndex, RowIndex};

    #[test]
    fn ability_meta_cooldown_for_level_returns_none_for_zero() {
        let meta = AbilityMeta::new(3, false, [10, 8, 6, 0]);
        assert_eq!(meta.cooldown_for_level(0), None);
    }

    #[test]
    fn ability_meta_cooldown_for_level_returns_none_beyond_max() {
        let meta = AbilityMeta::new(3, false, [10, 8, 6, 0]);
        assert_eq!(meta.cooldown_for_level(4), None);
    }

    #[test]
    fn ability_meta_cooldown_for_valid_levels() {
        let meta = AbilityMeta::new(3, false, [10, 8, 6, 0]);
        assert_eq!(meta.cooldown_for_level(1), Some(10));
        assert_eq!(meta.cooldown_for_level(2), Some(8));
        assert_eq!(meta.cooldown_for_level(3), Some(6));
    }

    #[test]
    fn ability_meta_base_cooldown_is_level_one() {
        let meta = AbilityMeta::new(3, false, [15, 10, 5, 0]);
        assert_eq!(meta.base_cooldown(), 15);
    }

    #[test]
    fn ability_meta_with_morph_target_stores_id() {
        let target = WarcraftObjectId::new("Hamg");
        let meta = AbilityMeta::new(1, false, [0; 4]).with_morph_target(Some(target));
        assert_eq!(meta.morph_target_unit().map(|id| id.value()), Some("Hamg"));
    }

    #[test]
    fn ability_meta_with_off_state_stores_all_fields() {
        let position = GridCoordinate::new(ColumnIndex::Three, RowIndex::Two);
        let meta = AbilityMeta::new(1, false, [0; 4]).with_off_state(
            Some(position),
            Some("Stop Defending"),
            Some("Deactivates defend"),
            Some("passivebuttons/btndefend.blp"),
        );
        let expected_coordinate = GridCoordinate::new(ColumnIndex::Three, RowIndex::Two);
        assert_eq!(meta.off_button_position(), Some(expected_coordinate));
        assert_eq!(meta.off_tip(), Some("Stop Defending"));
        assert_eq!(meta.off_ubertip(), Some("Deactivates defend"));
        assert_eq!(meta.off_icon(), Some("passivebuttons/btndefend.blp"));
    }
}
