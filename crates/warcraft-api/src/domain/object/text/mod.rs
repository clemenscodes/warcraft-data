//! [`WarcraftObjectText`]: the tooltip strings attached to an object.

/// Display text (tooltips) attached to an object. `un_*` variants are the
/// "off"/alternate-state strings for toggleable objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WarcraftObjectText {
    pub(crate) tip_levels: &'static [&'static str],
    pub(crate) ubertip_levels: &'static [&'static str],
    pub(crate) un_tip: Option<&'static str>,
    pub(crate) un_ubertip: Option<&'static str>,
}

impl WarcraftObjectText {
    pub const fn new(
        tip_levels: &'static [&'static str],
        ubertip_levels: &'static [&'static str],
    ) -> Self {
        Self {
            tip_levels,
            ubertip_levels,
            un_tip: None,
            un_ubertip: None,
        }
    }

    pub const fn with_alt(
        tip_levels: &'static [&'static str],
        ubertip_levels: &'static [&'static str],
        un_tip: Option<&'static str>,
        un_ubertip: Option<&'static str>,
    ) -> Self {
        Self {
            tip_levels,
            ubertip_levels,
            un_tip,
            un_ubertip,
        }
    }

    pub fn tip_levels(&self) -> &'static [&'static str] {
        self.tip_levels
    }

    pub fn ubertip_levels(&self) -> &'static [&'static str] {
        self.ubertip_levels
    }

    pub fn un_tip(&self) -> Option<&'static str> {
        self.un_tip
    }

    pub fn un_ubertip(&self) -> Option<&'static str> {
        self.un_ubertip
    }
}

// DDD role: immutable tooltip text, equality-by-value → Value Object.
impl ddd::Layered for WarcraftObjectText {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for WarcraftObjectText {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warcraft_object_text_accessors_return_slices() {
        let text = WarcraftObjectText::new(&["tip one", "tip two"], &["ubertip"]);
        assert_eq!(text.tip_levels(), &["tip one", "tip two"]);
        assert_eq!(text.ubertip_levels(), &["ubertip"]);
        assert!(text.un_tip().is_none());
    }

    #[test]
    fn warcraft_object_text_with_alt_stores_optional_fields() {
        let text = WarcraftObjectText::with_alt(&[], &[], Some("un tip"), Some("un uber"));
        assert_eq!(text.un_tip(), Some("un tip"));
        assert_eq!(text.un_ubertip(), Some("un uber"));
    }
}
