//! [`UnitModeSelection`]: which game modes a browse lists. The two modes are
//! independent, not a switch — melee and campaign units are findable together,
//! so a player looking for a unit never has to know which half of the game it
//! lives in first. A query input, so it lives in the application layer with
//! named fields — never a positional `(bool, bool)`.

use crate::domain::unit::{UnitMeta, UnitMode};
use std::fmt;
use std::str::FromStr;

/// The game modes a listing admits. Both may be on at once, and both may be
/// off, which admits nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitModeSelection {
    /// List the units available in melee.
    pub melee: bool,
    /// List the units available in campaign.
    pub campaign: bool,
}

impl UnitModeSelection {
    /// Both modes at once: the widest listing, and what a search always uses.
    pub fn both() -> Self {
        Self {
            melee: true,
            campaign: true,
        }
    }

    /// Only the given mode.
    pub fn only(mode: UnitMode) -> Self {
        match mode {
            UnitMode::Melee => Self {
                melee: true,
                campaign: false,
            },
            UnitMode::Campaign => Self {
                melee: false,
                campaign: true,
            },
        }
    }

    /// Whether this selection admits nothing, and so lists nothing.
    pub fn is_empty(&self) -> bool {
        !self.melee && !self.campaign
    }

    /// Whether the given mode is selected.
    pub fn includes(&self, mode: UnitMode) -> bool {
        match mode {
            UnitMode::Melee => self.melee,
            UnitMode::Campaign => self.campaign,
        }
    }

    /// The selection with one mode flipped.
    ///
    /// Turning the last mode off would list nothing at all, which is never what
    /// a player means by clicking a filter, so the last one on stays on. That is
    /// the one place the two modes are not fully independent, and it is a floor,
    /// not the old either/or.
    pub fn toggled(&self, mode: UnitMode) -> Self {
        let mut next = *self;
        let turning_off = self.includes(mode);
        match mode {
            UnitMode::Melee => next.melee = !turning_off,
            UnitMode::Campaign => next.campaign = !turning_off,
        }
        if next.is_empty() {
            return *self;
        }
        next
    }

    /// Whether the unit is available in any selected mode.
    pub fn admits(&self, meta: &UnitMeta) -> bool {
        if self.melee && meta.passes_filter(UnitMode::Melee) {
            return true;
        }
        if self.campaign && meta.passes_filter(UnitMode::Campaign) {
            return true;
        }
        false
    }
}

impl Default for UnitModeSelection {
    /// Melee alone, the mode a fresh browse opens on.
    fn default() -> Self {
        Self::only(UnitMode::Melee)
    }
}

impl From<UnitMode> for UnitModeSelection {
    fn from(mode: UnitMode) -> Self {
        Self::only(mode)
    }
}

/// The modes, comma-separated, in a fixed order — the form the URL carries.
/// An empty selection writes the empty string.
impl fmt::Display for UnitModeSelection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut written_any = false;
        if self.melee {
            write!(formatter, "{}", UnitMode::Melee)?;
            written_any = true;
        }
        if self.campaign {
            if written_any {
                formatter.write_str(",")?;
            }
            write!(formatter, "{}", UnitMode::Campaign)?;
        }
        Ok(())
    }
}

/// Reads what [`Display`](fmt::Display) wrote, and is deliberately forgiving of
/// what a URL may hold: unknown names are ignored rather than failing, so a
/// hand-edited or stale link degrades to whatever it did name instead of an
/// error page. It only fails when nothing at all was recognised, which lets the
/// caller fall back to a default rather than silently listing nothing.
impl FromStr for UnitModeSelection {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut selection = Self {
            melee: false,
            campaign: false,
        };
        for name in value.split(',') {
            let trimmed = name.trim();
            let Ok(mode) = UnitMode::try_from(trimmed) else {
                continue;
            };
            match mode {
                UnitMode::Melee => selection.melee = true,
                UnitMode::Campaign => selection.campaign = true,
            }
        }
        if selection.is_empty() {
            return Err(());
        }
        Ok(selection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggling_a_mode_on_keeps_the_other_one_on() {
        let melee_only = UnitModeSelection::only(UnitMode::Melee);
        let both = melee_only.toggled(UnitMode::Campaign);
        assert_eq!(
            both,
            UnitModeSelection::both(),
            "the modes are independent: turning campaign on must not turn melee off"
        );
    }

    #[test]
    fn toggling_a_mode_off_leaves_the_other_one_selected() {
        let both = UnitModeSelection::both();
        let campaign_only = both.toggled(UnitMode::Melee);
        assert_eq!(campaign_only, UnitModeSelection::only(UnitMode::Campaign));
    }

    #[test]
    fn toggling_off_the_last_mode_is_refused() {
        let melee_only = UnitModeSelection::only(UnitMode::Melee);
        let unchanged = melee_only.toggled(UnitMode::Melee);
        assert_eq!(
            unchanged, melee_only,
            "clicking a filter never means list nothing at all"
        );
    }

    #[test]
    fn includes_reports_each_mode() {
        let campaign_only = UnitModeSelection::only(UnitMode::Campaign);
        assert!(!campaign_only.includes(UnitMode::Melee));
        assert!(campaign_only.includes(UnitMode::Campaign));
    }

    #[test]
    fn a_selection_round_trips_through_its_text_form() {
        let cases = [
            UnitModeSelection::only(UnitMode::Melee),
            UnitModeSelection::only(UnitMode::Campaign),
            UnitModeSelection::both(),
        ];
        for selection in cases {
            let written = selection.to_string();
            let parsed = UnitModeSelection::from_str(&written)
                .expect("a non-empty selection must parse back");
            assert_eq!(parsed, selection, "round trip failed for {written:?}");
        }
    }

    #[test]
    fn both_modes_write_a_comma_separated_pair() {
        let both = UnitModeSelection::both();
        assert_eq!(both.to_string(), "melee,campaign");
    }

    #[test]
    fn an_empty_selection_writes_nothing_and_does_not_parse_back() {
        let empty = UnitModeSelection {
            melee: false,
            campaign: false,
        };
        assert_eq!(empty.to_string(), "");
        assert!(
            UnitModeSelection::from_str("").is_err(),
            "nothing recognised must fail so the caller can fall back to a default"
        );
    }

    #[test]
    fn unknown_names_are_ignored_rather_than_failing() {
        let parsed = UnitModeSelection::from_str("melee,nonsense")
            .expect("a stale link that still names one real mode keeps that mode");
        assert_eq!(parsed, UnitModeSelection::only(UnitMode::Melee));
    }

    #[test]
    fn a_single_mode_parses_from_its_own_name() {
        let parsed = UnitModeSelection::from_str("campaign").expect("campaign parses");
        assert_eq!(parsed, UnitModeSelection::only(UnitMode::Campaign));
    }

    #[test]
    fn only_one_mode_selects_exactly_that_mode() {
        let melee = UnitModeSelection::only(UnitMode::Melee);
        assert!(melee.melee);
        assert!(!melee.campaign);
        let campaign = UnitModeSelection::only(UnitMode::Campaign);
        assert!(!campaign.melee);
        assert!(campaign.campaign);
    }

    #[test]
    fn both_selects_both_modes() {
        let selection = UnitModeSelection::both();
        assert!(selection.melee);
        assert!(selection.campaign);
        assert!(!selection.is_empty());
    }

    #[test]
    fn a_selection_with_no_mode_is_empty() {
        let selection = UnitModeSelection {
            melee: false,
            campaign: false,
        };
        assert!(selection.is_empty());
    }

    #[test]
    fn the_default_is_melee_alone() {
        let selection = UnitModeSelection::default();
        let melee_only = UnitModeSelection::only(UnitMode::Melee);
        assert_eq!(
            selection, melee_only,
            "a fresh browse opens on melee, as it did when the mode was a single value"
        );
    }

    #[test]
    fn a_single_mode_converts_into_the_selection_holding_it() {
        let converted = UnitModeSelection::from(UnitMode::Campaign);
        let expected = UnitModeSelection::only(UnitMode::Campaign);
        assert_eq!(converted, expected);
    }
}
