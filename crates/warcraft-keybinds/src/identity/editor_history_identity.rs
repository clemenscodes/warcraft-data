/// The identity of the one [`crate::EditorHistory`] aggregate. There is exactly one
/// `EditorHistory` per editor, so its identity carries no data: every value of this type
/// names that same aggregate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct EditorHistoryIdentity;

impl ddd::Layered for EditorHistoryIdentity {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for EditorHistoryIdentity {}

impl ddd::Identifier for EditorHistoryIdentity {}

#[cfg(test)]
mod ddd_marker_tests {
    use super::EditorHistoryIdentity;
    use crate::ddd_conformance::assert_identifier;

    #[test]
    fn editor_history_identity_is_an_identifier() {
        assert_identifier::<EditorHistoryIdentity>();
    }
}
