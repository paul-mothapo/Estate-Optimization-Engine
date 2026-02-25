use super::ApiJurisdiction;
use crate::core::rules::tax_rules::{
    CapitalGainsAtDeathRule, DonationsTaxRule, EstateDutyRule, Jurisdiction,
    JurisdictionTaxRuleSet, TaxRuleRegistryEntry, TaxRuleVersion, VersionedJurisdictionTaxRuleSet,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JurisdictionTaxRuleRegistryResponse {
    pub jurisdiction: Jurisdiction,
    pub versions: Vec<TaxRuleVersion>,
    pub supported_tax_year_from: u16,
    pub supported_tax_year_to: Option<u16>,
    pub latest_version_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ApiTaxRuleVersion {
    pub version_id: String,
    pub tax_year_from: u16,
    pub tax_year_to: Option<u16>,
    pub effective_from: String,
    pub effective_to: Option<String>,
    pub source_last_verified_on: String,
}

impl From<TaxRuleVersion> for ApiTaxRuleVersion {
    fn from(value: TaxRuleVersion) -> Self {
        ApiTaxRuleVersion {
            version_id: value.version_id.to_string(),
            tax_year_from: value.tax_year_from,
            tax_year_to: value.tax_year_to,
            effective_from: value.effective_from.to_string(),
            effective_to: value.effective_to.map(str::to_string),
            source_last_verified_on: value.source_last_verified_on.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ApiTaxRuleRegistryEntry {
    pub jurisdiction: ApiJurisdiction,
    pub version: ApiTaxRuleVersion,
}

impl From<TaxRuleRegistryEntry> for ApiTaxRuleRegistryEntry {
    fn from(value: TaxRuleRegistryEntry) -> Self {
        ApiTaxRuleRegistryEntry {
            jurisdiction: value.jurisdiction.into(),
            version: value.version.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ApiJurisdictionTaxRuleRegistryResponse {
    pub jurisdiction: ApiJurisdiction,
    pub versions: Vec<ApiTaxRuleVersion>,
    pub supported_tax_year_from: u16,
    pub supported_tax_year_to: Option<u16>,
    pub latest_version_id: String,
}

impl From<JurisdictionTaxRuleRegistryResponse> for ApiJurisdictionTaxRuleRegistryResponse {
    fn from(value: JurisdictionTaxRuleRegistryResponse) -> Self {
        ApiJurisdictionTaxRuleRegistryResponse {
            jurisdiction: value.jurisdiction.into(),
            versions: value
                .versions
                .into_iter()
                .map(ApiTaxRuleVersion::from)
                .collect(),
            supported_tax_year_from: value.supported_tax_year_from,
            supported_tax_year_to: value.supported_tax_year_to,
            latest_version_id: value.latest_version_id.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiEstateDutyRule {
    pub section_4a_abatement_zar: f64,
    pub primary_rate: f64,
    pub primary_rate_cap_zar: f64,
    pub secondary_rate: f64,
    pub spouse_deduction_unlimited: bool,
    pub effective_from: String,
    pub source: String,
    pub source_url: String,
}

impl From<EstateDutyRule> for ApiEstateDutyRule {
    fn from(value: EstateDutyRule) -> Self {
        ApiEstateDutyRule {
            section_4a_abatement_zar: value.section_4a_abatement_zar,
            primary_rate: value.primary_rate,
            primary_rate_cap_zar: value.primary_rate_cap_zar,
            secondary_rate: value.secondary_rate,
            spouse_deduction_unlimited: value.spouse_deduction_unlimited,
            effective_from: value.effective_from.to_string(),
            source: value.source.to_string(),
            source_url: value.source_url.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiDonationsTaxRule {
    pub annual_exemption_natural_person_zar: f64,
    pub annual_exemption_non_natural_casual_gifts_zar: f64,
    pub primary_rate: f64,
    pub primary_rate_cap_cumulative_zar: f64,
    pub secondary_rate: f64,
    pub effective_from: String,
    pub source: String,
    pub source_url: String,
}

impl From<DonationsTaxRule> for ApiDonationsTaxRule {
    fn from(value: DonationsTaxRule) -> Self {
        ApiDonationsTaxRule {
            annual_exemption_natural_person_zar: value.annual_exemption_natural_person_zar,
            annual_exemption_non_natural_casual_gifts_zar: value
                .annual_exemption_non_natural_casual_gifts_zar,
            primary_rate: value.primary_rate,
            primary_rate_cap_cumulative_zar: value.primary_rate_cap_cumulative_zar,
            secondary_rate: value.secondary_rate,
            effective_from: value.effective_from.to_string(),
            source: value.source.to_string(),
            source_url: value.source_url.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiCapitalGainsAtDeathRule {
    pub annual_exclusion_in_year_of_death_zar: f64,
    pub inclusion_rate_natural_person: f64,
    pub inclusion_rate_company: f64,
    pub inclusion_rate_trust: f64,
    pub base_cost_step_up_to_market_value_on_death: bool,
    pub effective_from: String,
    pub source: String,
    pub source_url: String,
}

impl From<CapitalGainsAtDeathRule> for ApiCapitalGainsAtDeathRule {
    fn from(value: CapitalGainsAtDeathRule) -> Self {
        ApiCapitalGainsAtDeathRule {
            annual_exclusion_in_year_of_death_zar: value.annual_exclusion_in_year_of_death_zar,
            inclusion_rate_natural_person: value.inclusion_rate_natural_person,
            inclusion_rate_company: value.inclusion_rate_company,
            inclusion_rate_trust: value.inclusion_rate_trust,
            base_cost_step_up_to_market_value_on_death: value
                .base_cost_step_up_to_market_value_on_death,
            effective_from: value.effective_from.to_string(),
            source: value.source.to_string(),
            source_url: value.source_url.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiJurisdictionTaxRuleSet {
    pub estate_duty: ApiEstateDutyRule,
    pub donations_tax: ApiDonationsTaxRule,
    pub cgt_on_death: ApiCapitalGainsAtDeathRule,
}

impl From<JurisdictionTaxRuleSet> for ApiJurisdictionTaxRuleSet {
    fn from(value: JurisdictionTaxRuleSet) -> Self {
        ApiJurisdictionTaxRuleSet {
            estate_duty: value.estate_duty.into(),
            donations_tax: value.donations_tax.into(),
            cgt_on_death: value.cgt_on_death.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiVersionedJurisdictionTaxRuleSet {
    pub version: ApiTaxRuleVersion,
    pub rules: ApiJurisdictionTaxRuleSet,
}

impl From<VersionedJurisdictionTaxRuleSet> for ApiVersionedJurisdictionTaxRuleSet {
    fn from(value: VersionedJurisdictionTaxRuleSet) -> Self {
        ApiVersionedJurisdictionTaxRuleSet {
            version: value.version.into(),
            rules: value.rules.into(),
        }
    }
}
