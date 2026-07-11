//! The extractor's own intermediate representation.
//!
//! The extractor is a code generator: it must never mint sealed
//! `warcraft_api::WarcraftObjectId` values (its constructor is `pub(crate)` to
//! `warcraft-api`). Instead the build side materializes these owned mirror
//! structs — carrying raw `String`/`Vec<String>` ids — and the codegen walks
//! them, emitting `WarcraftObjectId::new("...")` as source text (which compiles
//! inside `warcraft-api`, where the constructor is reachable).
//!
//! Only id-bearing shapes are mirrored. Pure value types with no id fields
//! (`UnitCombat`, `UnitAttack`, `ManaPool`, `HeroAttributes`, and the enums) are
//! reused directly from `warcraft-api`.
//!
//! `ExtractedObjectId` reproduces `WarcraftObjectId`'s case-insensitive
//! `Eq`/`Ord`/`Hash` exactly. This is load-bearing: the codegen emits objects in
//! `BTreeMap` key order and dedupes by key equality, so a case-sensitive `String`
//! key would reorder the generated file and change which duplicates collapse.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use warcraft_api::{
    GridCoordinate, HeroAttributes, ItemClass, Race, UnitCombat, UnitKind, WarcraftObjectKind,
};

pub type ExtractedObjectMap = BTreeMap<ExtractedObjectId, ExtractedObject>;

/// An owned object id. Mirrors `warcraft_api::WarcraftObjectId`'s
/// case-insensitive equality and ordering so the generated output is
/// byte-identical to materializing real ids.
#[derive(Default, Debug, Clone)]
pub struct ExtractedObjectId {
    value: String,
}

impl ExtractedObjectId {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl From<&str> for ExtractedObjectId {
    fn from(value: &str) -> Self {
        let owned = value.to_string();
        Self::new(owned)
    }
}

impl PartialEq for ExtractedObjectId {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq_ignore_ascii_case(&other.value)
    }
}

impl Eq for ExtractedObjectId {}

impl Hash for ExtractedObjectId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for byte in self.value.bytes() {
            let folded = byte.to_ascii_lowercase();
            state.write_u8(folded);
        }
        state.write_u8(0xff);
    }
}

impl PartialOrd for ExtractedObjectId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExtractedObjectId {
    fn cmp(&self, other: &Self) -> Ordering {
        let own = self.value.bytes().map(|byte| byte.to_ascii_lowercase());
        let their = other.value.bytes().map(|byte| byte.to_ascii_lowercase());
        own.cmp(their)
    }
}

/// The generated database: the sorted object map the codegen walks.
#[derive(Default, Debug, Clone)]
pub struct ExtractedDatabase {
    db: ExtractedObjectMap,
}

impl ExtractedDatabase {
    pub fn new(db: ExtractedObjectMap) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &ExtractedObjectMap {
        &self.db
    }

    /// Case-insensitive lookup by raw id string. Mirrors
    /// `warcraft_api::WarcraftDatabase::by_id`; the map key already folds case.
    pub fn by_id(&self, needle_id: &str) -> Option<&ExtractedObject> {
        let key = ExtractedObjectId::from(needle_id);
        self.db.get(&key)
    }
}

/// Owned mirror of `warcraft_api::WarcraftObject`. Getter names and return
/// semantics match, except id-bearing accessors yield `&str` instead of
/// `WarcraftObjectId`.
#[derive(Debug, Clone)]
pub struct ExtractedObject {
    id: ExtractedObjectId,
    names: Vec<String>,
    icons: Vec<String>,
    kind: WarcraftObjectKind,
    race: Option<Race>,
    meta: ExtractedMeta,
    tip_levels: Vec<String>,
    ubertip_levels: Vec<String>,
    un_tip: Option<String>,
    un_ubertip: Option<String>,
    default_button_position: Option<GridCoordinate>,
    default_research_button_position: Option<GridCoordinate>,
}

impl ExtractedObject {
    pub fn new(fields: ExtractedObjectFields) -> Self {
        Self {
            id: fields.id,
            names: fields.names,
            icons: fields.icons,
            kind: fields.kind,
            race: fields.race,
            meta: fields.meta,
            tip_levels: fields.tip_levels,
            ubertip_levels: fields.ubertip_levels,
            un_tip: fields.un_tip,
            un_ubertip: fields.un_ubertip,
            default_button_position: fields.default_button_position,
            default_research_button_position: fields.default_research_button_position,
        }
    }

    pub fn id(&self) -> &ExtractedObjectId {
        &self.id
    }

    pub fn names(&self) -> &[String] {
        &self.names
    }

    pub fn icons(&self) -> &[String] {
        &self.icons
    }

    pub fn kind(&self) -> WarcraftObjectKind {
        self.kind
    }

    pub fn race(&self) -> Option<Race> {
        self.race
    }

    pub fn meta(&self) -> &ExtractedMeta {
        &self.meta
    }

    pub fn tip_levels(&self) -> &[String] {
        &self.tip_levels
    }

    pub fn ubertip_levels(&self) -> &[String] {
        &self.ubertip_levels
    }

    pub fn un_tip(&self) -> Option<&str> {
        self.un_tip.as_deref()
    }

    pub fn un_ubertip(&self) -> Option<&str> {
        self.un_ubertip.as_deref()
    }

    pub fn default_button_position(&self) -> Option<GridCoordinate> {
        if self.default_button_position.is_some() {
            return self.default_button_position;
        }
        match &self.meta {
            ExtractedMeta::Ability(ability_meta) => ability_meta.default_button_position(),
            ExtractedMeta::Command(command_meta) => command_meta.default_button_position(),
            _ => None,
        }
    }

    pub fn default_research_button_position(&self) -> Option<GridCoordinate> {
        if self.default_research_button_position.is_some() {
            return self.default_research_button_position;
        }
        match &self.meta {
            ExtractedMeta::Ability(ability_meta) => ability_meta.default_research_button_position(),
            _ => None,
        }
    }
}

/// Construction bundle for `ExtractedObject`, so the build side assigns a named
/// struct rather than passing a long positional argument list.
#[derive(Debug, Clone)]
pub struct ExtractedObjectFields {
    pub id: ExtractedObjectId,
    pub names: Vec<String>,
    pub icons: Vec<String>,
    pub kind: WarcraftObjectKind,
    pub race: Option<Race>,
    pub meta: ExtractedMeta,
    pub tip_levels: Vec<String>,
    pub ubertip_levels: Vec<String>,
    pub un_tip: Option<String>,
    pub un_ubertip: Option<String>,
    pub default_button_position: Option<GridCoordinate>,
    pub default_research_button_position: Option<GridCoordinate>,
}

/// Owned mirror of `warcraft_api::WarcraftObjectMeta`.
#[derive(Debug, Clone)]
pub enum ExtractedMeta {
    Unit(ExtractedUnitMeta),
    Ability(ExtractedAbilityMeta),
    Upgrade(ExtractedUpgradeMeta),
    Command(ExtractedCommandMeta),
    Item(ExtractedItemMeta),
}

/// Owned mirror of `warcraft_api::UnitMeta`. Combat and hero attributes reuse
/// the id-free `warcraft-api` value types directly.
#[derive(Debug, Clone)]
pub struct ExtractedUnitMeta {
    unit_kind: UnitKind,
    build_time: u32,
    abilities: Vec<String>,
    hero_abilities: Vec<String>,
    researches: Vec<String>,
    builds: Vec<String>,
    trains: Vec<String>,
    sell_items: Vec<String>,
    sell_units: Vec<String>,
    is_campaign: bool,
    is_in_editor: bool,
    is_hidden_in_editor: bool,
    is_special: bool,
    combat: UnitCombat,
    level: u32,
    gold_cost: u32,
    hero_attributes: Option<HeroAttributes>,
}

impl ExtractedUnitMeta {
    pub fn new(fields: ExtractedUnitMetaFields) -> Self {
        Self {
            unit_kind: fields.unit_kind,
            build_time: fields.build_time,
            abilities: fields.abilities,
            hero_abilities: fields.hero_abilities,
            researches: fields.researches,
            builds: fields.builds,
            trains: fields.trains,
            sell_items: fields.sell_items,
            sell_units: fields.sell_units,
            is_campaign: fields.is_campaign,
            is_in_editor: fields.is_in_editor,
            is_hidden_in_editor: fields.is_hidden_in_editor,
            is_special: fields.is_special,
            combat: fields.combat,
            level: fields.level,
            gold_cost: fields.gold_cost,
            hero_attributes: fields.hero_attributes,
        }
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

    pub fn abilities(&self) -> &[String] {
        &self.abilities
    }

    pub fn hero_abilities(&self) -> &[String] {
        &self.hero_abilities
    }

    pub fn researches(&self) -> &[String] {
        &self.researches
    }

    pub fn builds(&self) -> &[String] {
        &self.builds
    }

    pub fn trains(&self) -> &[String] {
        &self.trains
    }

    pub fn sell_items(&self) -> &[String] {
        &self.sell_items
    }

    pub fn sell_units(&self) -> &[String] {
        &self.sell_units
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

    pub fn combat(&self) -> &UnitCombat {
        &self.combat
    }

    pub fn hero_attributes(&self) -> Option<&HeroAttributes> {
        self.hero_attributes.as_ref()
    }
}

/// Construction bundle for `ExtractedUnitMeta`.
#[derive(Debug, Clone)]
pub struct ExtractedUnitMetaFields {
    pub unit_kind: UnitKind,
    pub build_time: u32,
    pub abilities: Vec<String>,
    pub hero_abilities: Vec<String>,
    pub researches: Vec<String>,
    pub builds: Vec<String>,
    pub trains: Vec<String>,
    pub sell_items: Vec<String>,
    pub sell_units: Vec<String>,
    pub is_campaign: bool,
    pub is_in_editor: bool,
    pub is_hidden_in_editor: bool,
    pub is_special: bool,
    pub combat: UnitCombat,
    pub level: u32,
    pub gold_cost: u32,
    pub hero_attributes: Option<HeroAttributes>,
}

/// Owned mirror of `warcraft_api::AbilityMeta`.
#[derive(Debug, Clone)]
pub struct ExtractedAbilityMeta {
    max_level: usize,
    is_ultimate: bool,
    cooldowns: [u32; 4],
    default_button_position: Option<GridCoordinate>,
    default_research_button_position: Option<GridCoordinate>,
    ubertip: Option<String>,
    research_ubertip: Option<String>,
    code: Option<String>,
    morph_target_unit: Option<String>,
    evasion_chances: [f32; 4],
    off_button_position: Option<GridCoordinate>,
    off_tip: Option<String>,
    off_ubertip: Option<String>,
    off_icon: Option<String>,
}

impl ExtractedAbilityMeta {
    pub fn new(fields: ExtractedAbilityMetaFields) -> Self {
        Self {
            max_level: fields.max_level,
            is_ultimate: fields.is_ultimate,
            cooldowns: fields.cooldowns,
            default_button_position: fields.default_button_position,
            default_research_button_position: fields.default_research_button_position,
            ubertip: fields.ubertip,
            research_ubertip: fields.research_ubertip,
            code: fields.code,
            morph_target_unit: fields.morph_target_unit,
            evasion_chances: fields.evasion_chances,
            off_button_position: fields.off_button_position,
            off_tip: fields.off_tip,
            off_ubertip: fields.off_ubertip,
            off_icon: fields.off_icon,
        }
    }

    pub fn max_level(&self) -> usize {
        self.max_level
    }

    pub fn is_ultimate(&self) -> bool {
        self.is_ultimate
    }

    pub fn cooldowns(&self) -> [u32; 4] {
        self.cooldowns
    }

    pub fn default_button_position(&self) -> Option<GridCoordinate> {
        self.default_button_position
    }

    pub fn default_research_button_position(&self) -> Option<GridCoordinate> {
        self.default_research_button_position
    }

    pub fn ubertip(&self) -> Option<&str> {
        self.ubertip.as_deref()
    }

    pub fn research_ubertip(&self) -> Option<&str> {
        self.research_ubertip.as_deref()
    }

    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    pub fn morph_target_unit(&self) -> Option<&str> {
        self.morph_target_unit.as_deref()
    }

    pub fn evasion_chances(&self) -> [f32; 4] {
        self.evasion_chances
    }

    pub fn off_button_position(&self) -> Option<GridCoordinate> {
        self.off_button_position
    }

    pub fn off_tip(&self) -> Option<&str> {
        self.off_tip.as_deref()
    }

    pub fn off_ubertip(&self) -> Option<&str> {
        self.off_ubertip.as_deref()
    }

    pub fn off_icon(&self) -> Option<&str> {
        self.off_icon.as_deref()
    }
}

/// Construction bundle for `ExtractedAbilityMeta`.
#[derive(Debug, Clone)]
pub struct ExtractedAbilityMetaFields {
    pub max_level: usize,
    pub is_ultimate: bool,
    pub cooldowns: [u32; 4],
    pub default_button_position: Option<GridCoordinate>,
    pub default_research_button_position: Option<GridCoordinate>,
    pub ubertip: Option<String>,
    pub research_ubertip: Option<String>,
    pub code: Option<String>,
    pub morph_target_unit: Option<String>,
    pub evasion_chances: [f32; 4],
    pub off_button_position: Option<GridCoordinate>,
    pub off_tip: Option<String>,
    pub off_ubertip: Option<String>,
    pub off_icon: Option<String>,
}

/// Owned mirror of `warcraft_api::UpgradeMeta`.
#[derive(Debug, Clone)]
pub struct ExtractedUpgradeMeta {
    max_level: usize,
}

impl ExtractedUpgradeMeta {
    pub fn new(max_level: usize) -> Self {
        Self { max_level }
    }

    pub fn max_level(&self) -> usize {
        self.max_level
    }
}

/// Owned mirror of `warcraft_api::CommandMeta`.
#[derive(Debug, Clone)]
pub struct ExtractedCommandMeta {
    default_button_position: Option<GridCoordinate>,
    tip: Option<String>,
    ubertip: Option<String>,
}

impl ExtractedCommandMeta {
    pub fn new(
        default_button_position: Option<GridCoordinate>,
        tip: Option<String>,
        ubertip: Option<String>,
    ) -> Self {
        Self {
            default_button_position,
            tip,
            ubertip,
        }
    }

    pub fn default_button_position(&self) -> Option<GridCoordinate> {
        self.default_button_position
    }

    pub fn tip(&self) -> Option<&str> {
        self.tip.as_deref()
    }

    pub fn ubertip(&self) -> Option<&str> {
        self.ubertip.as_deref()
    }
}

/// Owned mirror of `warcraft_api::ItemMeta`.
#[derive(Debug, Clone)]
pub struct ExtractedItemMeta {
    class: ItemClass,
    abilities: Vec<String>,
    cooldown_id: Option<String>,
}

impl ExtractedItemMeta {
    pub fn new(class: ItemClass, abilities: Vec<String>, cooldown_id: Option<String>) -> Self {
        Self {
            class,
            abilities,
            cooldown_id,
        }
    }

    pub fn class(&self) -> &ItemClass {
        &self.class
    }

    pub fn abilities(&self) -> &[String] {
        &self.abilities
    }

    pub fn cooldown_id(&self) -> Option<&str> {
        self.cooldown_id.as_deref()
    }
}
