//! The typed scalar figures that make up a unit's combat statistics. Each wraps a
//! single primitive (or a small, inseparable pair) so the value carries its meaning
//! through the type system: a `HitPoints` can never be mistaken for a `Mana`, and no
//! caller can pass a raw `u32` where a stat is expected. Presentation — formatting a
//! figure into a display string, choosing a colour — is the renderer's job, never
//! the domain's; these types carry only the value.

use crate::domain::unit::RegenType;

/// A unit's maximum hit points.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct HitPoints {
    value: u32,
}

impl HitPoints {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn value(self) -> u32 {
        self.value
    }
}

/// Hit-point regeneration per second, together with the condition under which it
/// applies (only at night, only on blight, or always).
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct HitPointsRegen {
    value: f32,
    regen_type: RegenType,
}

impl HitPointsRegen {
    pub fn new(value: f32, regen_type: RegenType) -> Self {
        Self { value, regen_type }
    }

    pub fn value(self) -> f32 {
        self.value
    }

    pub fn regen_type(self) -> RegenType {
        self.regen_type
    }

    /// Whether there is no regeneration (zero or negative), so the figure renders muted.
    pub fn is_zero(self) -> bool {
        self.value <= 0.0
    }
}

/// A unit's maximum mana. Zero for a unit with no mana pool.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct Mana {
    value: u32,
}

impl Mana {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn value(self) -> u32 {
        self.value
    }

    /// Whether this unit has no mana pool (a mana of zero), so the figure renders muted.
    pub fn is_zero(self) -> bool {
        self.value == 0
    }
}

/// Mana regeneration per second.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct ManaRegen {
    value: f32,
}

impl ManaRegen {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(self) -> f32 {
        self.value
    }

    /// Whether there is no regeneration (zero or negative), so the figure renders muted.
    pub fn is_zero(self) -> bool {
        self.value <= 0.0
    }
}

/// A unit's armor, as a derived combat figure. Fractional and can be negative
/// (which amplifies incoming damage). Named `ArmorFigure` to distinguish it from
/// the stored [`crate::Armor`] quantity (an `Eq` fixed-point value); this is the
/// computed `f32` figure the statistics card shows.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct ArmorFigure {
    value: f32,
}

impl ArmorFigure {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(self) -> f32 {
        self.value
    }
}

/// Effective hit points: how much raw incoming damage the unit survives once its
/// hit points are scaled by armor and evasion. A derived figure, computed by
/// [`super::UnitStatistics`].
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct EffectiveHitPoints {
    value: f32,
}

impl EffectiveHitPoints {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(self) -> f32 {
        self.value
    }
}

/// A unit's evasion chance, in the range `0.0..=1.0` (the fraction of attacks it
/// dodges). Zero for a unit with no evasion source.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct Evasion {
    chance: f32,
}

impl Evasion {
    pub fn new(chance: f32) -> Self {
        Self { chance }
    }

    pub fn chance(self) -> f32 {
        self.chance
    }

    /// The evasion as a percentage in `0.0..=100.0` — the form the figure is shown in.
    pub fn percent(self) -> f32 {
        self.chance * 100.0
    }

    /// Whether the unit has no evasion source (a chance of zero), so no evasion row
    /// is shown.
    pub fn is_zero(self) -> bool {
        self.chance <= 0.0
    }
}

/// An attack's damage span, from minimum to maximum roll.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct DamageRange {
    minimum: u32,
    maximum: u32,
}

impl DamageRange {
    pub fn new(minimum: u32, maximum: u32) -> Self {
        Self { minimum, maximum }
    }

    pub fn minimum(self) -> u32 {
        self.minimum
    }

    pub fn maximum(self) -> u32 {
        self.maximum
    }
}

/// An attack's reach, in game distance units.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct AttackRange {
    value: u32,
}

impl AttackRange {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn value(self) -> u32 {
        self.value
    }

    /// Whether the attack is melee (a reach of zero), so no range row is shown.
    pub fn is_zero(self) -> bool {
        self.value == 0
    }
}

/// An attack's cooldown, in seconds between swings.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct AttackSpeed {
    cooldown_seconds: f32,
}

impl AttackSpeed {
    pub fn new(cooldown_seconds: f32) -> Self {
        Self { cooldown_seconds }
    }

    pub fn cooldown_seconds(self) -> f32 {
        self.cooldown_seconds
    }
}

/// Mean damage per second over the attack cooldown. A derived figure, computed by
/// [`super::UnitStatistics`].
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct DamagePerSecond {
    value: f32,
}

impl DamagePerSecond {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(self) -> f32 {
        self.value
    }
}

/// A positive increment shown with a leading `+`: a regeneration rate (per second)
/// or a per-level attribute growth. The renderer renders it as `+{value}` and mutes
/// it when it is zero.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct Gain {
    value: f32,
}

impl Gain {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(self) -> f32 {
        self.value
    }

    /// Whether there is no gain (zero or negative), so the figure renders muted.
    pub fn is_zero(self) -> bool {
        self.value <= 0.0
    }
}
