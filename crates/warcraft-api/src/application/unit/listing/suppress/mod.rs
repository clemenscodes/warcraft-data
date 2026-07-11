//! Pure fuzzy suppression: a fuzzy-only match survives only when nothing matched
//! directly anywhere in the whole candidate set. This is a global decision, so
//! it takes the full set of match outcomes, not one candidate at a time.

use crate::application::unit::listing::search::Match;

/// Whether any candidate matched directly.
pub(crate) fn any_direct(matches: &[Match]) -> bool {
    matches.contains(&Match::Direct)
}

/// Whether a candidate with this match outcome survives, given whether any
/// direct match exists in the set. Direct always survives; fuzzy survives only
/// when there is no direct match anywhere; a non-match never survives.
pub(crate) fn survives(outcome: Match, any_direct: bool) -> bool {
    match outcome {
        Match::Direct => true,
        Match::Fuzzy => !any_direct,
        Match::None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_direct_is_true_when_a_direct_match_is_present() {
        assert!(any_direct(&[Match::Fuzzy, Match::Direct]));
    }

    #[test]
    fn any_direct_is_false_with_only_fuzzy_matches() {
        assert!(!any_direct(&[Match::Fuzzy, Match::Fuzzy]));
    }

    #[test]
    fn direct_always_survives() {
        assert!(survives(Match::Direct, true));
        assert!(survives(Match::Direct, false));
    }

    #[test]
    fn fuzzy_survives_only_without_any_direct() {
        assert!(!survives(Match::Fuzzy, true));
        assert!(survives(Match::Fuzzy, false));
    }

    #[test]
    fn a_non_match_never_survives() {
        assert!(!survives(Match::None, false));
    }
}
