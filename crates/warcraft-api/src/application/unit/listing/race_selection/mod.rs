//! [`RaceSelection`]: which races a listing admits. A browse narrows to the one
//! active race; a search scopes across a chosen set, defaulting to every race.
//! A query input, so it lives in the application layer with named fields, never
//! a positional flag.

use crate::domain::race::Race;
use std::collections::BTreeSet;

/// The races a listing admits. `All` admits every object, including the rare
/// object that has no race. `Only` admits an object only when its race is one of
/// the named set, so a raceless object is excluded, exactly as a single-race
/// browse excluded it before.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RaceSelection {
    /// Every race, and every raceless object. The default and the widest scope.
    All,
    /// Only the named races.
    Only { races: BTreeSet<Race> },
}

impl RaceSelection {
    /// Only the one race. The scope a browse uses.
    pub fn only(race: Race) -> Self {
        let mut races = BTreeSet::new();
        races.insert(race);
        Self::Only { races }
    }

    /// Only the named races, gathered from any iterator. An empty iterator
    /// yields an `Only` that admits nothing.
    pub fn of(races: impl IntoIterator<Item = Race>) -> Self {
        let races: BTreeSet<Race> = races.into_iter().collect();
        Self::Only { races }
    }

    /// Whether the selection admits an object with this race. A raceless object
    /// (`None`) is admitted only by `All`.
    pub fn admits(&self, race: Option<Race>) -> bool {
        match self {
            Self::All => true,
            Self::Only { races } => match race {
                Some(race) => races.contains(&race),
                None => false,
            },
        }
    }

    /// Whether this selection names no race at all, and so admits nothing.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::All => false,
            Self::Only { races } => races.is_empty(),
        }
    }
}

impl Default for RaceSelection {
    /// Every race, the scope a fresh search opens on.
    fn default() -> Self {
        Self::All
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_is_all_races() {
        let selection = RaceSelection::default();
        assert_eq!(selection, RaceSelection::All);
    }

    #[test]
    fn all_admits_every_race_and_the_raceless() {
        let selection = RaceSelection::All;
        assert!(selection.admits(Some(Race::Human)));
        assert!(selection.admits(Some(Race::Neutral)));
        assert!(
            selection.admits(None),
            "an object with no race is part of a cross-race search"
        );
    }

    #[test]
    fn only_admits_the_named_race_and_rejects_the_others() {
        let selection = RaceSelection::only(Race::Nightelf);
        assert!(selection.admits(Some(Race::Nightelf)));
        assert!(!selection.admits(Some(Race::Human)));
    }

    #[test]
    fn only_rejects_a_raceless_object() {
        let selection = RaceSelection::only(Race::Nightelf);
        assert!(
            !selection.admits(None),
            "a named race scope excludes objects that have no race, as a single-race browse did"
        );
    }

    #[test]
    fn of_gathers_several_races() {
        let selection = RaceSelection::of([Race::Human, Race::Undead]);
        assert!(selection.admits(Some(Race::Human)));
        assert!(selection.admits(Some(Race::Undead)));
        assert!(!selection.admits(Some(Race::Orc)));
    }

    #[test]
    fn an_empty_only_admits_nothing() {
        let selection = RaceSelection::of(std::iter::empty::<Race>());
        assert!(selection.is_empty());
        assert!(!selection.admits(Some(Race::Human)));
    }

    #[test]
    fn all_is_never_empty() {
        let selection = RaceSelection::All;
        assert!(!selection.is_empty());
    }
}
