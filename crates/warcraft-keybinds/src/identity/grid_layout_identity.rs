/// The identity of the one [`crate::GridLayout`] aggregate. There is exactly one
/// `GridLayout` per editor, so its identity carries no data: every value of this type
/// names that same aggregate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct GridLayoutIdentity;

impl ddd::Layered for GridLayoutIdentity {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for GridLayoutIdentity {}

impl ddd::Identifier for GridLayoutIdentity {}

#[cfg(test)]
mod ddd_marker_tests {
    use super::GridLayoutIdentity;
    use crate::ddd_conformance::assert_identifier;

    #[test]
    fn grid_layout_identity_is_an_identifier() {
        assert_identifier::<GridLayoutIdentity>();
    }
}
