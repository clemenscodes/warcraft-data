//! [`AbilityView`]: the flat, read-only result of an ability query. Holds no
//! database handle; the reverse edge (which units carry it) is resolved by the
//! caller through [`AbilityApi`](crate::AbilityApi).

use crate::domain::ability::AbilityMeta;
use crate::domain::identity::WarcraftObjectId;
use crate::domain::object::{WarcraftObject, WarcraftObjectMeta};

/// A single ability, as returned by an [`AbilityApi`](crate::AbilityApi) query.
/// A `Copy` handle over the immutable stored object.
#[derive(Clone, Copy, Debug)]
pub struct AbilityView {
    object: &'static WarcraftObject,
    meta: &'static AbilityMeta,
}

impl AbilityView {
    pub fn id(&self) -> WarcraftObjectId {
        self.object.id()
    }

    /// The primary display name, if any.
    pub fn name(&self) -> Option<&'static str> {
        self.object.names().first().copied()
    }

    /// The primary icon path, if any.
    pub fn icon(&self) -> Option<&'static str> {
        self.object.icons().first().copied()
    }

    /// All icon paths.
    pub fn icons(&self) -> &'static [&'static str] {
        self.object.icons()
    }

    /// The ability's game-mechanic class (`code` column of `abilitydata.slk`).
    pub fn code(&self) -> Option<WarcraftObjectId> {
        self.meta.code()
    }
}

/// Views a stored object as an ability — succeeds only when it is an ability.
impl TryFrom<&'static WarcraftObject> for AbilityView {
    type Error = ();

    fn try_from(object: &'static WarcraftObject) -> Result<Self, Self::Error> {
        let WarcraftObjectMeta::Ability(meta) = object.meta() else {
            return Err(());
        };
        Ok(Self { object, meta })
    }
}

// DDD role: a read model returned by the ability application service.
impl ddd::ReadModel for AbilityView {}
