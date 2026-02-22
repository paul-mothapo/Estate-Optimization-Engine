use crate::jurisdictions::south_africa::{
    south_africa_latest_tax_rules,
    south_africa_tax_rules_for_year,
};
use std::fmt;

// Country rates/thresholds are owned by jurisdiction modules.
// Keep these core types country-agnostic and route via `baseline_tax_rules_for`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxPayerClass {
    NaturalPerson,
    Company,
    Trust,
    SpecialTrust,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Jurisdiction {
    SouthAfrica,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaxRuleVersion {
    pub version_id: &'static str,
    pub tax_year_from: u16,
    pub tax_year_to: Option<u16>,
    pub effective_from: &'static str,
    pub effective_to: Option<&'static str>,
    pub source_last_verified_on: &'static str,
}

#[derive(Debug, Clone)]
pub struct EstateDutyRule {
    pub section_4a_abatement_zar: f64,
    pub primary_rate: f64,
    pub primary_rate_cap_zar: f64,
    pub secondary_rate: f64,
    pub spouse_deduction_unlimited: bool,
    pub effective_from: &'static str,
    pub source: &'static str,
    pub source_url: &'static str,
}

#[derive(Debug, Clone)]
pub struct DonationsTaxRule {
    pub annual_exemption_natural_person_zar: f64,
    pub annual_exemption_non_natural_casual_gifts_zar: f64,
    pub primary_rate: f64,
    pub primary_rate_cap_cumulative_zar: f64,
    pub secondary_rate: f64,
    pub effective_from: &'static str,
    pub source: &'static str,
    pub source_url: &'static str,
}

#[derive(Debug, Clone)]
pub struct CapitalGainsAtDeathRule {
    pub annual_exclusion_in_year_of_death_zar: f64,
    pub inclusion_rate_natural_person: f64,
    pub inclusion_rate_company: f64,
    pub inclusion_rate_trust: f64,
    pub base_cost_step_up_to_market_value_on_death: bool,
    pub effective_from: &'static str,
    pub source: &'static str,
    pub source_url: &'static str,
}

impl CapitalGainsAtDeathRule {
    pub fn inclusion_rate_for(&self, taxpayer: TaxPayerClass) -> f64 {
        match taxpayer {
            TaxPayerClass::NaturalPerson | TaxPayerClass::SpecialTrust => {
                self.inclusion_rate_natural_person
            }
            TaxPayerClass::Company => self.inclusion_rate_company,
            TaxPayerClass::Trust => self.inclusion_rate_trust,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JurisdictionTaxRuleSet {
    pub estate_duty: EstateDutyRule,
    pub donations_tax: DonationsTaxRule,
    pub cgt_on_death: CapitalGainsAtDeathRule,
}

#[derive(Debug, Clone)]
pub struct VersionedJurisdictionTaxRuleSet {
    pub version: TaxRuleVersion,
    pub rules: JurisdictionTaxRuleSet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxRuleSelectionError {
    UnsupportedTaxYear {
        jurisdiction: Jurisdiction,
        tax_year: u16,
    },
}

impl fmt::Display for TaxRuleSelectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaxRuleSelectionError::UnsupportedTaxYear {
                jurisdiction,
                tax_year,
            } => write!(
                f,
                "No tax rule version found for jurisdiction {:?} and tax year {}",
                jurisdiction, tax_year
            ),
        }
    }
}

impl std::error::Error for TaxRuleSelectionError {}

pub fn tax_rules_for(
    jurisdiction: Jurisdiction,
    tax_year: u16,
) -> Result<VersionedJurisdictionTaxRuleSet, TaxRuleSelectionError> {
    match jurisdiction {
        Jurisdiction::SouthAfrica => south_africa_tax_rules_for_year(tax_year),
    }
}

pub fn latest_tax_rules_for(jurisdiction: Jurisdiction) -> VersionedJurisdictionTaxRuleSet {
    match jurisdiction {
        Jurisdiction::SouthAfrica => south_africa_latest_tax_rules(),
    }
}

pub fn baseline_tax_rules_for(jurisdiction: Jurisdiction) -> JurisdictionTaxRuleSet {
    latest_tax_rules_for(jurisdiction).rules
}

pub fn is_supported_tax_year(jurisdiction: Jurisdiction, tax_year: u16) -> bool {
    tax_rules_for(jurisdiction, tax_year).is_ok()
}
