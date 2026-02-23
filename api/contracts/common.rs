use crate::core::rules::tax_rules::Jurisdiction;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ApiJurisdiction {
    SouthAfrica,
}

impl From<Jurisdiction> for ApiJurisdiction {
    fn from(value: Jurisdiction) -> Self {
        match value {
            Jurisdiction::SouthAfrica => ApiJurisdiction::SouthAfrica,
        }
    }
}

impl From<ApiJurisdiction> for Jurisdiction {
    fn from(value: ApiJurisdiction) -> Self {
        match value {
            ApiJurisdiction::SouthAfrica => Jurisdiction::SouthAfrica,
        }
    }
}
