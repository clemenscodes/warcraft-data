//! [`WarcraftObjectKind`]: which kind of game object a `WarcraftObject` is.

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarcraftObjectKind {
    #[default]
    Unit,
    Ability,
    Upgrade,
    Item,
    Command,
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for WarcraftObjectKind {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for WarcraftObjectKind {}
