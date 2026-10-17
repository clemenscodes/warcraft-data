/// The identity of the one [`crate::CustomKeys`] aggregate. There is exactly one
/// `CustomKeys` per editor, so its identity carries no data: every value of this type
/// names that same aggregate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CustomKeysIdentity;

impl ddd::Layered for CustomKeysIdentity {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for CustomKeysIdentity {}

impl ddd::Identifier for CustomKeysIdentity {}

#[cfg(test)]
mod ddd_marker_tests {
    use super::CustomKeysIdentity;
    use crate::ddd_conformance::assert_identifier;

    #[test]
    fn custom_keys_identity_is_an_identifier() {
        assert_identifier::<CustomKeysIdentity>();
    }
}
