use crate::identity::ability_id::AbilityId;
use crate::identity::keycode::KeyCode;
use crate::identity::slot::GridSlotId;

use crate::model::{
    AbilityBinding, BindingEntry, CommandBinding, CommandEntry, GridCoordinate, Hotkey,
    SystemBinding, WarcraftKeybinding,
};

use std::collections::BTreeMap;
use std::fmt;
use warcraft_api::WarcraftObjectId;

mod mutate;
mod normalize;
mod overlay;
mod parser;
mod resolve;

#[cfg(test)]
mod tests;

use parser::CustomKeysParser;

pub const DEFAULT_CUSTOM_KEYS: &str = include_str!("../../templates/CustomKeys.txt");
const BUNDLED_BASELINE: &str = DEFAULT_CUSTOM_KEYS;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HotkeyConflict {
    display_name: String,
}

impl HotkeyConflict {
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}

const GRID_COLUMNS: u8 = 4;
const GRID_ROWS: u8 = 3;

#[derive(Clone, Debug, Default)]
pub struct CustomKeys {
    entries: BTreeMap<WarcraftObjectId, WarcraftKeybinding>,
}

impl From<BTreeMap<WarcraftObjectId, WarcraftKeybinding>> for CustomKeys {
    fn from(entries: BTreeMap<WarcraftObjectId, WarcraftKeybinding>) -> Self {
        Self { entries }
    }
}

impl ddd::Layered for CustomKeys {
    type Layer = ddd::DomainLayer;
}

impl ddd::AggregateRoot for CustomKeys {}

#[cfg(test)]
mod ddd_marker_tests {
    use super::CustomKeys;
    use ddd::AggregateRoot;
    use ddd::DomainLayer;
    use ddd::Layered;

    fn assert_domain_aggregate<Aggregate>()
    where
        Aggregate: AggregateRoot + Layered<Layer = DomainLayer>,
    {
    }

    #[test]
    fn custom_keys_is_a_domain_aggregate_root() {
        assert_domain_aggregate::<CustomKeys>();
    }

    #[test]
    fn from_text_is_idempotent() {
        let once = CustomKeys::from_text("");
        let reparsed = CustomKeys::from_text(&once.to_string());
        assert_eq!(once.to_string(), reparsed.to_string());
    }
}

impl CustomKeys {
    pub fn binding(&self, id: impl Into<AbilityId>) -> Option<&AbilityBinding> {
        let ability_id = id.into();
        let object_id = ability_id.object_id();
        self.entries.get(&object_id)?.as_ability()
    }

    pub(crate) fn binding_or_default_mut(
        &mut self,
        id: impl Into<AbilityId>,
    ) -> Option<&mut AbilityBinding> {
        let ability_id = id.into();
        let object_id = ability_id.object_id();
        if !matches!(
            self.entries.get(&object_id),
            Some(WarcraftKeybinding::Ability(_))
        ) {
            let default_binding = AbilityBinding::default();
            self.entries
                .insert(object_id, WarcraftKeybinding::Ability(default_binding));
        }
        self.entries
            .get_mut(&object_id)
            .and_then(WarcraftKeybinding::as_ability_mut)
    }

    pub fn bindings_in_order(&self) -> impl Iterator<Item = BindingEntry<'_>> {
        self.entries.iter().filter_map(|(id, binding)| {
            binding.as_ability().map(|ability| {
                let ability_id = AbilityId::from(*id);
                BindingEntry::new(ability_id, ability)
            })
        })
    }

    pub fn command(&self, name: impl Into<WarcraftObjectId>) -> Option<&CommandBinding> {
        let object_id = name.into();
        self.entries.get(&object_id)?.as_command()
    }

    pub(crate) fn command_or_default_mut(
        &mut self,
        name: impl Into<WarcraftObjectId>,
    ) -> Option<&mut CommandBinding> {
        let object_id = name.into();
        if !matches!(
            self.entries.get(&object_id),
            Some(WarcraftKeybinding::Command(_))
        ) {
            let default_binding = CommandBinding::default();
            self.entries
                .insert(object_id, WarcraftKeybinding::Command(default_binding));
        }
        self.entries
            .get_mut(&object_id)
            .and_then(WarcraftKeybinding::as_command_mut)
    }

    pub fn commands_in_order(&self) -> impl Iterator<Item = CommandEntry<'_>> {
        self.entries.iter().filter_map(|(name, binding)| {
            binding
                .as_command()
                .map(|command| CommandEntry::new(*name, command))
        })
    }

    pub fn system(&self, id: impl Into<WarcraftObjectId>) -> Option<&SystemBinding> {
        let object_id = id.into();
        self.entries.get(&object_id)?.as_system()
    }

    pub(crate) fn system_mut(
        &mut self,
        id: impl Into<WarcraftObjectId>,
    ) -> Option<&mut SystemBinding> {
        let object_id = id.into();
        self.entries.get_mut(&object_id)?.as_system_mut()
    }

    pub fn set_system_hotkey(&mut self, section_id: impl Into<WarcraftObjectId>, key: KeyCode) {
        let section_object_id = section_id.into();
        let hotkey_code = u32::from(key);
        let hotkey = Hotkey::VirtualKey(hotkey_code);
        if let Some(binding) = self.system_mut(section_object_id) {
            binding.set_hotkey(hotkey);
        }
    }

    pub fn builder() -> crate::model::CustomKeysBuilder {
        crate::model::CustomKeysBuilder::default()
    }

    pub fn put_ability(&mut self, id: impl Into<AbilityId>, binding: AbilityBinding) {
        let ability_id = id.into();
        let object_id = ability_id.object_id();
        self.entries
            .insert(object_id, WarcraftKeybinding::Ability(binding));
    }

    pub fn put_command(&mut self, name: impl Into<WarcraftObjectId>, binding: CommandBinding) {
        let object_id = name.into();
        self.entries
            .insert(object_id, WarcraftKeybinding::Command(binding));
    }

    pub fn put_system(&mut self, id: impl Into<WarcraftObjectId>, binding: SystemBinding) {
        let object_id = id.into();
        self.entries
            .insert(object_id, WarcraftKeybinding::System(binding));
    }

    pub fn swap_system_bindings(
        &mut self,
        source_id: impl Into<WarcraftObjectId>,
        target_id: impl Into<WarcraftObjectId>,
    ) {
        let source_object_id = source_id.into();
        let target_object_id = target_id.into();
        let source_hotkey =
            self.system(source_object_id)
                .and_then(|binding| match binding.hotkey() {
                    Hotkey::VirtualKey(code) => Some(*code),
                    _ => None,
                });
        let target_hotkey =
            self.system(target_object_id)
                .and_then(|binding| match binding.hotkey() {
                    Hotkey::VirtualKey(code) => Some(*code),
                    _ => None,
                });
        if let Some(binding) = self.system_mut(source_object_id) {
            binding.set_hotkey(Hotkey::VirtualKey(target_hotkey.unwrap_or(0)));
        }
        if let Some(binding) = self.system_mut(target_object_id) {
            binding.set_hotkey(Hotkey::VirtualKey(source_hotkey.unwrap_or(0)));
        }
    }

    pub fn position_for_slot(
        &self,
        slot: &GridSlotId,
        is_research_context: bool,
    ) -> Option<GridCoordinate> {
        match slot {
            GridSlotId::Ability(ability_id) => {
                let bound_id = *ability_id;
                let binding = self.binding(bound_id)?;
                if is_research_context {
                    binding.research_button_position().copied()
                } else {
                    binding.button_position().copied()
                }
            }
            GridSlotId::AbilityOff(ability_id) => {
                let bound_id = *ability_id;
                let binding = self.binding(bound_id)?;
                binding.unbutton_position().copied()
            }
            GridSlotId::Command(command_name) => {
                let binding = self.command(*command_name)?;
                binding.button_position().copied()
            }
        }
    }

    pub(crate) fn slot_at_position(
        &self,
        slots: &[GridSlotId],
        is_research_context: bool,
        column: u8,
        row: u8,
    ) -> Option<GridSlotId> {
        for slot in slots {
            let Some(position) = self.position_for_slot(slot, is_research_context) else {
                continue;
            };
            let position_column = u8::from(position.column());
            let position_row = u8::from(position.row());
            if position_column == column && position_row == row {
                return Some(*slot);
            }
        }
        None
    }
}

impl fmt::Display for CustomKeys {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (object_id, entry) in &self.entries {
            match entry {
                WarcraftKeybinding::Ability(binding) => {
                    binding.write_section(formatter, *object_id)?;
                }
                WarcraftKeybinding::Command(binding) => {
                    binding.write_section(formatter, *object_id)?;
                }
                WarcraftKeybinding::System(binding) => {
                    binding.write_section(formatter, *object_id)?;
                }
            }
        }
        Ok(())
    }
}

impl IntoIterator for CustomKeys {
    type Item = (WarcraftObjectId, WarcraftKeybinding);
    type IntoIter = std::collections::btree_map::IntoIter<WarcraftObjectId, WarcraftKeybinding>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl CustomKeys {
    /// Parses `CustomKeys.txt` text into the raw entry map without normalizing.
    /// Internal only: the materialized baseline and the parser tests need the
    /// un-materialized parse, and [`CustomKeys::from_text`] builds on it. Every
    /// public path yields a normalized aggregate, so this stays `pub(crate)`.
    pub(crate) fn parse_raw(text: &str) -> Self {
        let mut parser = CustomKeysParser::new();
        for line in text.lines() {
            parser.process_line(line);
        }
        parser.finish()
    }

    /// Parses `CustomKeys.txt` text and normalizes it — the sole public
    /// constructor from text. There is no public way to obtain a non-normalized
    /// `CustomKeys`, so the type itself is the proof that its invariants hold.
    pub fn from_text(text: &str) -> Self {
        let raw = Self::parse_raw(text);
        raw.normalize()
    }

    /// Overlays an imported `CustomKeys.txt` (a template or an uploaded file) onto
    /// the bundled baseline and returns the normalized result together with the
    /// counts of what the import defined. This is the domain home for the "import
    /// replaces, then normalize" rule (R7); the renderer only calls it.
    pub fn import_overlay(overlay_text: &str) -> ImportOutcome {
        let overlay = Self::parse_raw(overlay_text);
        let binding_count = overlay.bindings_in_order().count();
        let command_count = overlay.commands_in_order().count();
        let mut baseline = Self::parse_raw(DEFAULT_CUSTOM_KEYS);
        baseline.extend(overlay);
        let keys = baseline.normalize();
        ImportOutcome {
            keys,
            binding_count,
            command_count,
        }
    }
}

/// The result of [`CustomKeys::import_overlay`]: the normalized keys plus how many
/// ability and command bindings the imported file defined.
#[derive(Clone, Debug)]
pub struct ImportOutcome {
    keys: CustomKeys,
    binding_count: usize,
    command_count: usize,
}

impl ImportOutcome {
    pub fn into_keys(self) -> CustomKeys {
        self.keys
    }

    pub fn binding_count(&self) -> usize {
        self.binding_count
    }

    pub fn command_count(&self) -> usize {
        self.command_count
    }
}

impl TryFrom<&std::path::Path> for CustomKeys {
    type Error = std::io::Error;

    fn try_from(path: &std::path::Path) -> Result<Self, Self::Error> {
        let text = std::fs::read_to_string(path)?;
        Ok(Self::from_text(text.as_str()))
    }
}
