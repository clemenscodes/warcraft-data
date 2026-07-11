//! [`MatchType`] and its sub-kinds: how a game was set up (melee / custom /
//! campaign).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchType {
    Melee(MeleeMatchType),
    Custom(CustomMatchType),
    Campaign(CampaignMatchType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeleeMatchType {
    OneVsOne,
    TwoVsTwo,
    ThreeVsThree,
    FourVsFour,
    FreeForAll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomMatchType {
    DirectStrike,
    Legion,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CampaignMatchType {}

// DDD roles: match-setup value objects (equality-by-value).
impl ddd::Layered for MatchType {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for MatchType {}

impl ddd::Layered for MeleeMatchType {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for MeleeMatchType {}

impl ddd::Layered for CustomMatchType {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for CustomMatchType {}

impl ddd::Layered for CampaignMatchType {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for CampaignMatchType {}
