use crate::jurisdictions::south_africa::south_africa_tax_rules_baseline;

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

pub fn baseline_tax_rules_for(jurisdiction: Jurisdiction) -> JurisdictionTaxRuleSet {
    match jurisdiction {
        Jurisdiction::SouthAfrica => south_africa_tax_rules_baseline(),
    }
}
