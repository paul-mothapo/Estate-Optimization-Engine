use crate::core::rules::tax_rules::{
    CapitalGainsAtDeathRule, DonationsTaxRule, EstateDutyRule, Jurisdiction,
    JurisdictionTaxRuleSet, TaxRuleSelectionError, TaxRuleVersion, VersionedJurisdictionTaxRuleSet,
};

#[derive(Debug, Clone)]
pub struct LegalInstrument {
    pub name: &'static str,
    pub citation: &'static str,
    pub planning_relevance: &'static str,
    pub source_url: &'static str,
}

#[derive(Debug, Clone)]
pub struct UsStateTaxSummary {
    pub tax_year_context: &'static str,
    pub rates_last_verified_on: &'static str,
    pub estate_tax_exemption_usd: f64,
    pub estate_tax_effective_top_rate: f64,
    pub annual_gift_exclusion_usd: f64,
    pub cgt_at_death_inclusion_rate_natural_person: f64,
}

#[derive(Debug, Clone)]
pub struct UsStateJurisdictionLaw {
    pub jurisdiction_code: &'static str,
    pub jurisdiction_name: &'static str,
    pub legal_instruments: Vec<LegalInstrument>,
    pub baseline_tax_summary: UsStateTaxSummary,
    pub tax_source_urls: Vec<&'static str>,
    pub notes: Vec<&'static str>,
}

#[derive(Debug, Clone, Copy)]
struct UsStatePolicy {
    jurisdiction_code: &'static str,
    jurisdiction_name: &'static str,
    version_id: &'static str,
    estate_tax_exemption_usd: f64,
    estate_tax_effective_top_rate: f64,
    estate_tax_source: &'static str,
    estate_tax_source_url: &'static str,
}

fn us_state_policy(jurisdiction: Jurisdiction) -> Option<UsStatePolicy> {
    match jurisdiction {
        Jurisdiction::UsNewYork => Some(UsStatePolicy {
            jurisdiction_code: "US-NY",
            jurisdiction_name: "United States - New York",
            version_id: "US-NY-ESTATE-BASELINE-2026+",
            estate_tax_exemption_usd: 7_000_000.0,
            estate_tax_effective_top_rate: 0.56,
            estate_tax_source: "IRS + New York combined estate-tax planning baseline",
            estate_tax_source_url: "https://www.tax.ny.gov/bus/estate/",
        }),
        Jurisdiction::UsTexas => Some(UsStatePolicy {
            jurisdiction_code: "US-TX",
            jurisdiction_name: "United States - Texas",
            version_id: "US-TX-ESTATE-BASELINE-2026+",
            estate_tax_exemption_usd: 7_000_000.0,
            estate_tax_effective_top_rate: 0.40,
            estate_tax_source: "IRS federal estate-tax baseline (no Texas estate tax)",
            estate_tax_source_url:
                "https://www.irs.gov/businesses/small-businesses-self-employed/estate-tax",
        }),
        Jurisdiction::UsCalifornia => Some(UsStatePolicy {
            jurisdiction_code: "US-CA",
            jurisdiction_name: "United States - California",
            version_id: "US-CA-ESTATE-BASELINE-2026+",
            estate_tax_exemption_usd: 7_000_000.0,
            estate_tax_effective_top_rate: 0.40,
            estate_tax_source: "IRS federal estate-tax baseline (no California estate tax)",
            estate_tax_source_url:
                "https://www.irs.gov/businesses/small-businesses-self-employed/estate-tax",
        }),
        Jurisdiction::UsFlorida => Some(UsStatePolicy {
            jurisdiction_code: "US-FL",
            jurisdiction_name: "United States - Florida",
            version_id: "US-FL-ESTATE-BASELINE-2026+",
            estate_tax_exemption_usd: 7_000_000.0,
            estate_tax_effective_top_rate: 0.40,
            estate_tax_source: "IRS federal estate-tax baseline (no Florida estate tax)",
            estate_tax_source_url:
                "https://www.irs.gov/businesses/small-businesses-self-employed/estate-tax",
        }),
        Jurisdiction::UsMinnesota => Some(UsStatePolicy {
            jurisdiction_code: "US-MN",
            jurisdiction_name: "United States - Minnesota",
            version_id: "US-MN-ESTATE-BASELINE-2026+",
            estate_tax_exemption_usd: 3_000_000.0,
            estate_tax_effective_top_rate: 0.56,
            estate_tax_source: "IRS + Minnesota combined estate-tax planning baseline",
            estate_tax_source_url: "https://www.revenue.state.mn.us/estate-tax",
        }),
        Jurisdiction::SouthAfrica => None,
    }
}

fn us_state_tax_rules_2026_onwards(jurisdiction: Jurisdiction) -> VersionedJurisdictionTaxRuleSet {
    let policy = us_state_policy(jurisdiction)
        .expect("US state tax rules requested for non-US jurisdiction variant");

    VersionedJurisdictionTaxRuleSet {
        version: TaxRuleVersion {
            version_id: policy.version_id,
            tax_year_from: 2026,
            tax_year_to: None,
            effective_from: "2026-01-01",
            effective_to: None,
            source_last_verified_on: "2026-03-03",
        },
        rules: JurisdictionTaxRuleSet {
            estate_duty: EstateDutyRule {
                exemption_amount: policy.estate_tax_exemption_usd,
                primary_rate: policy.estate_tax_effective_top_rate,
                primary_rate_cap_amount: 1_000_000_000_000.0,
                secondary_rate: policy.estate_tax_effective_top_rate,
                spouse_deduction_unlimited: true,
                effective_from: "2026-01-01",
                source: policy.estate_tax_source,
                source_url: policy.estate_tax_source_url,
            },
            donations_tax: DonationsTaxRule {
                annual_exemption_natural_person_amount: 20_000.0,
                annual_exemption_non_natural_casual_gifts_amount: 0.0,
                primary_rate: 0.40,
                primary_rate_cap_cumulative_amount: 1_000_000_000_000.0,
                secondary_rate: 0.40,
                effective_from: "2026-01-01",
                source: "IRS gift-tax planning baseline",
                source_url:
                    "https://www.irs.gov/businesses/small-businesses-self-employed/gift-tax",
            },
            cgt_on_death: CapitalGainsAtDeathRule {
                annual_exclusion_in_year_of_death_amount: 0.0,
                inclusion_rate_natural_person: 0.0,
                inclusion_rate_company: 0.0,
                inclusion_rate_trust: 0.0,
                base_cost_step_up_to_market_value_on_death: true,
                effective_from: "2026-01-01",
                source: "IRS basis-of-assets step-up treatment at death",
                source_url: "https://www.irs.gov/publications/p559#en_US_2024_publink10009920",
            },
        },
    }
}

pub fn us_state_tax_rules_catalog(
    jurisdiction: Jurisdiction,
) -> Vec<VersionedJurisdictionTaxRuleSet> {
    if us_state_policy(jurisdiction).is_none() {
        return Vec::new();
    }

    vec![us_state_tax_rules_2026_onwards(jurisdiction)]
}

pub fn us_state_tax_rules_for_year(
    jurisdiction: Jurisdiction,
    tax_year: u16,
) -> Result<VersionedJurisdictionTaxRuleSet, TaxRuleSelectionError> {
    for versioned in us_state_tax_rules_catalog(jurisdiction) {
        let from_ok = tax_year >= versioned.version.tax_year_from;
        let to_ok = match versioned.version.tax_year_to {
            Some(to) => tax_year <= to,
            None => true,
        };
        if from_ok && to_ok {
            return Ok(versioned);
        }
    }

    Err(TaxRuleSelectionError::UnsupportedTaxYear {
        jurisdiction,
        tax_year,
    })
}

pub fn us_state_latest_tax_rules(jurisdiction: Jurisdiction) -> VersionedJurisdictionTaxRuleSet {
    us_state_tax_rules_2026_onwards(jurisdiction)
}

pub fn us_state_jurisdiction_baseline(
    jurisdiction: Jurisdiction,
) -> Option<UsStateJurisdictionLaw> {
    let policy = us_state_policy(jurisdiction)?;
    let rules = us_state_latest_tax_rules(jurisdiction).rules;

    Some(UsStateJurisdictionLaw {
        jurisdiction_code: policy.jurisdiction_code,
        jurisdiction_name: policy.jurisdiction_name,
        legal_instruments: vec![
            LegalInstrument {
                name: "Internal Revenue Code",
                citation: "26 U.S.C. Subtitle B",
                planning_relevance: "Federal estate and gift transfer-tax framework.",
                source_url:
                    "https://www.law.cornell.edu/uscode/text/26/subtitle-B/chapter-11",
            },
            LegalInstrument {
                name: "Internal Revenue Code",
                citation: "26 U.S.C. section 1014",
                planning_relevance: "Basis step-up framework at death for capital assets.",
                source_url: "https://www.law.cornell.edu/uscode/text/26/1014",
            },
            LegalInstrument {
                name: "State Estate Tax",
                citation: policy.estate_tax_source,
                planning_relevance:
                    "State-level estate-tax overlay where applicable in New York and Minnesota.",
                source_url: policy.estate_tax_source_url,
            },
        ],
        baseline_tax_summary: UsStateTaxSummary {
            tax_year_context: "Tax years from 2026 onwards",
            rates_last_verified_on: "2026-03-03",
            estate_tax_exemption_usd: rules.estate_duty.exemption_amount,
            estate_tax_effective_top_rate: rules.estate_duty.primary_rate,
            annual_gift_exclusion_usd: rules.donations_tax.annual_exemption_natural_person_amount,
            cgt_at_death_inclusion_rate_natural_person: rules.cgt_on_death.inclusion_rate_natural_person,
        },
        tax_source_urls: vec![
            rules.estate_duty.source_url,
            rules.donations_tax.source_url,
            rules.cgt_on_death.source_url,
        ],
        notes: vec![
            "US baselines are implemented as planning assumptions using a combined federal+state effective estate-tax rate per state.",
            "New York and Minnesota include a state estate-tax overlay in the effective estate-tax rate.",
            "Texas, California, and Florida model federal estate-tax only in this baseline.",
            "Capital gains at death are modeled with a basis step-up (no immediate CGT realization in this rule set).",
            "Validate final filing positions with US-licensed tax counsel and state-specific guidance.",
        ],
    })
}
