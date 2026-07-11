//! [`UnitView`]: the flat, read-only result of a unit query. Holds no database
//! handle and cannot navigate — relations are exposed as raw ids and resolved
//! by the caller through [`UnitApi`](crate::UnitApi).

use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::{WarcraftObject, WarcraftObjectMeta};
use crate::domain::race::Race;
use crate::domain::unit::{UnitKind, UnitMeta};

/// A single unit, as returned by a [`UnitApi`](crate::UnitApi) query. A `Copy`
/// handle over the immutable stored object; it can read its own fields but has
/// no way to reach any other object.
#[derive(Clone, Copy, Debug)]
pub struct UnitView {
    object: &'static WarcraftObject,
    meta: &'static UnitMeta,
}

impl UnitView {
    pub fn id(&self) -> WarcraftObjectId {
        self.object.id()
    }

    /// The primary display name, if any.
    pub fn name(&self) -> Option<&'static str> {
        self.object.names().first().copied()
    }

    /// All display names (a unit can carry alternate names).
    pub fn names(&self) -> &'static [&'static str] {
        self.object.names()
    }

    pub fn race(&self) -> Option<Race> {
        self.object.race()
    }

    pub fn kind(&self) -> UnitKind {
        self.meta.unit_kind()
    }

    /// The primary icon path, if any.
    pub fn icon(&self) -> Option<&'static str> {
        self.object.icons().first().copied()
    }

    /// All icon paths.
    pub fn icons(&self) -> &'static [&'static str] {
        self.object.icons()
    }

    /// Ids of the abilities this unit carries. Resolve to
    /// [`AbilityView`](crate::AbilityView)s via
    /// [`UnitApi::abilities`](crate::UnitApi::abilities).
    pub fn ability_ids(&self) -> &'static [WarcraftObjectId] {
        self.meta.abilities()
    }

    /// Ids of this unit's hero abilities (empty for non-heroes).
    pub fn hero_ability_ids(&self) -> &'static [WarcraftObjectId] {
        self.meta.hero_abilities()
    }
}

/// Views a stored object as a unit — succeeds only when the object is a unit.
impl TryFrom<&'static WarcraftObject> for UnitView {
    type Error = ();

    fn try_from(object: &'static WarcraftObject) -> Result<Self, Self::Error> {
        let WarcraftObjectMeta::Unit(meta) = object.meta() else {
            return Err(());
        };
        Ok(Self { object, meta })
    }
}

// DDD role: a read model returned by the unit application service.
impl ddd::ReadModel for UnitView {}
