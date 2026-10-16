//! [`TalentVariant`]: a hero talent paired with the hero ability it replaces.

use crate::domain::identity::WarcraftObjectId;

/// A hero talent (patch 3.0.0): a variant of a hero ability that replaces its
/// base on the same command card button once the talent is picked (e.g. Guiding
/// Hand `AHgh` and its talents `AHq1`..`AHq3`). The live game then reads the
/// variant's own `CustomKeys.txt` section, so the variant must carry its base's
/// keybind. Extracted from the talent rows of `units/abilitydata.slk`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TalentVariant {
    variant_id: WarcraftObjectId,
    base_id: WarcraftObjectId,
}

impl TalentVariant {
    pub(crate) const fn new(variant_id: &'static str, base_id: &'static str) -> Self {
        let variant_id = WarcraftObjectId::new(variant_id);
        let base_id = WarcraftObjectId::new(base_id);
        Self {
            variant_id,
            base_id,
        }
    }

    pub fn variant_id(&self) -> WarcraftObjectId {
        self.variant_id
    }

    pub fn base_id(&self) -> WarcraftObjectId {
        self.base_id
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for TalentVariant {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for TalentVariant {}
