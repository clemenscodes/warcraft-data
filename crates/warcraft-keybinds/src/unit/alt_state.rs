//! Keybind-local alt-state predicates: classify a unit's toggled alternate
//! (burrowed / rooted / militia) state and the abilities that render on it.
//! These are specific to command-card rendering, so they live here rather than
//! in the general `warcraft-api`. Implemented for [`WarcraftApi`] so they can
//! read the database; uprooting delegates to `api.unit().can_uproot`, which
//! already detects the `Aroo` root ability code.

use warcraft_api::{WarcraftApi, WarcraftObjectId, WarcraftObjectMeta};

/// Predicates classifying a unit's toggled alternate state and the abilities
/// that appear on it. Implemented for [`WarcraftApi`].
pub trait AltState {
    /// Whether the unit's first display name marks it as a burrowed form.
    fn is_burrowed_form(&self, unit_id: WarcraftObjectId) -> bool;

    /// Whether the unit's default in-game state is the toggled alternate: an
    /// uprootable building, a burrowed form, or the Militia (`hmil`).
    fn unit_starts_in_toggle_alt_state(&self, unit_id: WarcraftObjectId) -> bool;

    /// Whether the ability appears on any unit that starts in a toggled
    /// alternate state.
    fn ability_is_on_alt_state_unit(&self, ability_id: WarcraftObjectId) -> bool;

    /// Whether the ability carries alternate-state tooltip text (its `un_tip` /
    /// `un_ubertip`), the signal that it renders a distinct off-state.
    fn ability_has_alt_state(&self, ability_id: WarcraftObjectId) -> bool;
}

impl AltState for WarcraftApi {
    fn is_burrowed_form(&self, unit_id: WarcraftObjectId) -> bool {
        let Some(object) = self.object(unit_id) else {
            return false;
        };
        let Some(first_name) = object.names().first().copied() else {
            return false;
        };
        first_name.to_ascii_lowercase().starts_with("burrowed ")
    }

    fn unit_starts_in_toggle_alt_state(&self, unit_id: WarcraftObjectId) -> bool {
        self.unit().can_uproot(unit_id)
            || self.is_burrowed_form(unit_id)
            || self.resolve("hmil") == Some(unit_id)
    }

    fn ability_is_on_alt_state_unit(&self, ability_id: WarcraftObjectId) -> bool {
        self.iter().any(|(unit_id, object)| {
            if !self.unit_starts_in_toggle_alt_state(*unit_id) {
                return false;
            }
            matches!(
                object.meta(),
                WarcraftObjectMeta::Unit(unit_meta) if unit_meta.abilities().contains(&ability_id)
            )
        })
    }

    fn ability_has_alt_state(&self, ability_id: WarcraftObjectId) -> bool {
        let Some(object) = self.object(ability_id) else {
            return false;
        };
        object.un_tip().is_some() || object.un_ubertip().is_some()
    }
}
