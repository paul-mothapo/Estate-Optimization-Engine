use crate::core::rules::tax_rules::Jurisdiction;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ApiJurisdiction {
    SouthAfrica,
    UsNewYork,
    UsTexas,
    UsCalifornia,
    UsFlorida,
    UsMinnesota,
}

impl From<Jurisdiction> for ApiJurisdiction {
    fn from(value: Jurisdiction) -> Self {
        match value {
            Jurisdiction::SouthAfrica => ApiJurisdiction::SouthAfrica,
            Jurisdiction::UsNewYork => ApiJurisdiction::UsNewYork,
            Jurisdiction::UsTexas => ApiJurisdiction::UsTexas,
            Jurisdiction::UsCalifornia => ApiJurisdiction::UsCalifornia,
            Jurisdiction::UsFlorida => ApiJurisdiction::UsFlorida,
            Jurisdiction::UsMinnesota => ApiJurisdiction::UsMinnesota,
        }
    }
}

impl From<ApiJurisdiction> for Jurisdiction {
    fn from(value: ApiJurisdiction) -> Self {
        match value {
            ApiJurisdiction::SouthAfrica => Jurisdiction::SouthAfrica,
            ApiJurisdiction::UsNewYork => Jurisdiction::UsNewYork,
            ApiJurisdiction::UsTexas => Jurisdiction::UsTexas,
            ApiJurisdiction::UsCalifornia => Jurisdiction::UsCalifornia,
            ApiJurisdiction::UsFlorida => Jurisdiction::UsFlorida,
            ApiJurisdiction::UsMinnesota => Jurisdiction::UsMinnesota,
        }
    }
}
