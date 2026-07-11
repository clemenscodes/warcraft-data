//! Pure text-search matching: given a lowercased query and the lowercased text
//! to match against (a unit's names + id, or its ability haystack), decide
//! whether the query matches directly, only fuzzily, or not at all. No database,
//! no globals — every rule is a pure function of the strings passed in.

/// How a query matched a candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Match {
    /// A direct hit (substring or whole-word token match).
    Direct,
    /// Only a fuzzy (subsequence) hit — surfaced only when nothing matched
    /// directly anywhere in the result set.
    Fuzzy,
    /// No match; the candidate is dropped.
    None,
}

/// Match a query against a unit's own text: its space-joined lowercased names
/// and its lowercased id. Direct when a name/id substring or a ≥3-char query
/// token whole-word-matches a name word; otherwise fuzzy when every query char
/// appears in order within the names.
pub(crate) fn match_unit_name(query: &str, id_lower: &str, names_lower: &str) -> Match {
    let is_direct = names_lower.contains(query)
        || id_lower.contains(query)
        || query.contains(id_lower)
        || token_word_match(query, names_lower);
    if is_direct {
        return Match::Direct;
    }
    if is_subsequence(query, names_lower) {
        return Match::Fuzzy;
    }
    Match::None
}

/// Match a query against a unit's ability haystack (names + ids of its
/// button-positioned abilities). Direct only — no fuzzy fallback, since a
/// subsequence over concatenated ability text would match almost anything.
pub(crate) fn match_ability(query: &str, haystack: &str) -> Match {
    if haystack.contains(query) || token_word_match(query, haystack) {
        Match::Direct
    } else {
        Match::None
    }
}

/// Whether some ≥3-char whitespace token of `query` exactly equals a whitespace
/// word of `text`.
fn token_word_match(query: &str, text: &str) -> bool {
    query
        .split_whitespace()
        .filter(|token| token.len() >= 3)
        .any(|token| text.split_whitespace().any(|word| word == token))
}

/// Whether every character of `needle` appears in order within `haystack` (not
/// necessarily contiguously).
pub(crate) fn is_subsequence(needle: &str, haystack: &str) -> bool {
    let mut haystack_chars = haystack.chars();
    for needle_char in needle.chars() {
        loop {
            match haystack_chars.next() {
                Some(haystack_char) if haystack_char == needle_char => break,
                Some(_) => continue,
                None => return false,
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_name_substring_is_a_direct_match() {
        assert_eq!(match_unit_name("foot", "hfoo", "footman"), Match::Direct);
    }

    #[test]
    fn an_id_substring_is_a_direct_match() {
        assert_eq!(match_unit_name("hfoo", "hfoo", "footman"), Match::Direct);
    }

    #[test]
    fn a_short_query_token_does_not_word_match_but_can_subsequence() {
        // "wm" is under 3 chars so no token-word-match, but it is a subsequence
        // of "water elemental"? w..a..t..e..r -> w then m? no 'm' after 'w' start...
        // "wm": w (water) then m (eleMental) → subsequence.
        assert_eq!(
            match_unit_name("wm", "ewsp", "water elemental"),
            Match::Fuzzy
        );
    }

    #[test]
    fn an_unrelated_query_does_not_match() {
        assert_eq!(match_unit_name("zzzq", "hfoo", "footman"), Match::None);
    }

    #[test]
    fn ability_haystack_matches_directly_only() {
        // Substring hit → direct.
        assert_eq!(match_ability("slow", " slow acsw "), Match::Direct);
        // "sc" is a subsequence of the haystack but not a substring/word, and
        // abilities have no fuzzy fallback → no match.
        assert_eq!(match_ability("sc", " slow acsw "), Match::None);
    }

    #[test]
    fn is_subsequence_respects_order() {
        assert!(is_subsequence("ftm", "footman"));
        assert!(!is_subsequence("mtf", "footman"));
        assert!(is_subsequence("", "anything"));
    }
}
