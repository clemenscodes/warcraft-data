//! [`UpgradeMeta`]: how many research tiers an upgrade has.

/// Upgrade metadata: how many research tiers the upgrade has.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct UpgradeMeta {
    max_level: usize,
}

impl UpgradeMeta {
    pub fn new(max_level: usize) -> Self {
        Self { max_level }
    }

    pub fn max_level(&self) -> usize {
        self.max_level
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for UpgradeMeta {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for UpgradeMeta {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upgrade_meta_stores_max_level() {
        let meta = UpgradeMeta::new(3);
        assert_eq!(meta.max_level(), 3);
    }
}
