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
pub struct SouthAfricaJurisdictionLaw {
    pub jurisdiction_code: &'static str,
    pub jurisdiction_name: &'static str,
    pub legal_instruments: Vec<LegalInstrument>,
    pub baseline_tax_summary: SouthAfricaTaxSummary,
    pub tax_source_urls: Vec<&'static str>,
    pub notes: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub struct SouthAfricaTaxSummary {
    pub tax_year_context: &'static str,
    pub rates_last_verified_on: &'static str,
    pub estate_duty_rate_main: f64,
    pub estate_duty_rate_above_30m: f64,
    pub estate_duty_section_4a_abatement_zar: f64,
    pub donations_tax_rate_main: f64,
    pub donations_tax_rate_above_30m: f64,
    pub cgt_annual_exclusion_year_of_death_zar: f64,
}

fn summarize_tax_rules(rules: &JurisdictionTaxRuleSet) -> SouthAfricaTaxSummary {
    SouthAfricaTaxSummary {
        tax_year_context: "Tax years from 2018 onwards",
        rates_last_verified_on: "2026-02-21",
        estate_duty_rate_main: rules.estate_duty.primary_rate,
        estate_duty_rate_above_30m: rules.estate_duty.secondary_rate,
        estate_duty_section_4a_abatement_zar: rules.estate_duty.section_4a_abatement_zar,
        donations_tax_rate_main: rules.donations_tax.primary_rate,
        donations_tax_rate_above_30m: rules.donations_tax.secondary_rate,
        cgt_annual_exclusion_year_of_death_zar: rules
            .cgt_on_death
            .annual_exclusion_in_year_of_death_zar,
    }
}

fn south_africa_tax_rules_2018_onwards() -> VersionedJurisdictionTaxRuleSet {
    VersionedJurisdictionTaxRuleSet {
        version: TaxRuleVersion {
            version_id: "ZA-ESTATE-BASELINE-2018+",
            tax_year_from: 2018,
            tax_year_to: None,
            effective_from: "2018-03-01",
            effective_to: None,
            source_last_verified_on: "2026-02-21",
        },
        rules: JurisdictionTaxRuleSet {
            estate_duty: EstateDutyRule {
                section_4a_abatement_zar: 3_500_000.0,
                primary_rate: 0.20,
                primary_rate_cap_zar: 30_000_000.0,
                secondary_rate: 0.25,
                spouse_deduction_unlimited: true, // Estate Duty Act, section 4(q)
                effective_from: "2018-03-01",
                source: "SARS Estate Duty (accessed 2026-02-21)",
                source_url: "https://www.sars.gov.za/types-of-tax/estate-duty/",
            },
            donations_tax: DonationsTaxRule {
                annual_exemption_natural_person_zar: 100_000.0,
                annual_exemption_non_natural_casual_gifts_zar: 10_000.0,
                primary_rate: 0.20,
                primary_rate_cap_cumulative_zar: 30_000_000.0,
                secondary_rate: 0.25,
                effective_from: "2018-03-01",
                source: "SARS Donations Tax (accessed 2026-02-21)",
                source_url: "https://www.sars.gov.za/types-of-tax/donations-tax/",
            },
            cgt_on_death: CapitalGainsAtDeathRule {
                annual_exclusion_in_year_of_death_zar: 300_000.0,
                inclusion_rate_natural_person: 0.40,
                inclusion_rate_company: 0.80,
                inclusion_rate_trust: 0.80,
                base_cost_step_up_to_market_value_on_death: true,
                effective_from: "2016-03-01",
                source: "SARS CGT (page updated 2025-05-21; accessed 2026-02-21)",
                source_url: "https://www.sars.gov.za/tax-rates/income-tax/capital-gains-tax-cgt/",
            },
        },
    }
}

pub fn south_africa_tax_rules_catalog() -> Vec<VersionedJurisdictionTaxRuleSet> {
    vec![south_africa_tax_rules_2018_onwards()]
}

pub fn south_africa_tax_rules_for_year(
    tax_year: u16,
) -> Result<VersionedJurisdictionTaxRuleSet, TaxRuleSelectionError> {
    for versioned in south_africa_tax_rules_catalog() {
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
        jurisdiction: Jurisdiction::SouthAfrica,
        tax_year,
    })
}

pub fn south_africa_latest_tax_rules() -> VersionedJurisdictionTaxRuleSet {
    south_africa_tax_rules_2018_onwards()
}

pub fn south_africa_tax_rules_baseline() -> JurisdictionTaxRuleSet {
    south_africa_latest_tax_rules().rules
}

pub fn south_africa_jurisdiction_baseline() -> SouthAfricaJurisdictionLaw {
    let rules = south_africa_tax_rules_baseline();

    SouthAfricaJurisdictionLaw {
        jurisdiction_code: "ZA",
        jurisdiction_name: "South Africa",
        legal_instruments: vec![
            LegalInstrument {
                name: "Constitution of the Republic of South Africa",
                citation: "Constitution, 1996",
                planning_relevance: "Rights framework that underpins property, equality, and inheritance administration.",
                source_url: "https://www.justice.gov.za/legislation/constitution/",
            },
            LegalInstrument {
                name: "Wills Act",
                citation: "Act 7 of 1953",
                planning_relevance: "Formal validity of wills and execution requirements.",
                source_url: "https://www.saflii.org/za/legis/consol_act/wa195391/",
            },
            LegalInstrument {
                name: "Intestate Succession Act",
                citation: "Act 81 of 1987",
                planning_relevance: "Default inheritance order when no valid will exists.",
                source_url: "https://www.saflii.org/za/legis/consol_act/isa81o1987261/",
            },
            LegalInstrument {
                name: "Administration of Estates Act",
                citation: "Act 66 of 1965",
                planning_relevance: "Estate reporting, executor process, and liquidation/distribution administration.",
                source_url: "https://www.saflii.org/za/legis/consol_act/aoea1965274/",
            },
            LegalInstrument {
                name: "Estate Duty Act",
                citation: "Act 45 of 1955",
                planning_relevance: "Estate duty charging rules, deductions, and abatement.",
                source_url: "https://www.gov.za/documents/estate-duty-act-19-may-2015-1254",
            },
            LegalInstrument {
                name: "Income Tax Act",
                citation: "Act 58 of 1962",
                planning_relevance: "Capital gains tax deemed disposal at death and related exclusions.",
                source_url: "https://www.gov.za/documents/income-tax-act-29-may-1962-0000",
            },
            LegalInstrument {
                name: "Matrimonial Property Act",
                citation: "Act 88 of 1984",
                planning_relevance: "Marital property regime affects dutiable estate composition.",
                source_url: "https://www.saflii.org/za/legis/consol_act/mpa88o1984279/",
            },
            LegalInstrument {
                name: "Trust Property Control Act",
                citation: "Act 57 of 1988",
                planning_relevance: "Trust governance rules relevant to wealth transfer structures.",
                source_url: "https://www.gov.za/documents/trust-property-control-act-18-may-2015-1117",
            },
            LegalInstrument {
                name: "Pension Funds Act",
                citation: "Act 24 of 1956 (section 37C)",
                planning_relevance: "Death benefits are allocated by fund trustees, not strictly by will.",
                source_url: "https://www.saflii.org/za/legis/consol_act/pfa1956165/",
            },
        ],
        baseline_tax_summary: summarize_tax_rules(&rules),
        tax_source_urls: vec![
            rules.estate_duty.source_url,
            rules.donations_tax.source_url,
            rules.cgt_on_death.source_url,
        ],
        notes: vec![
            "Tax rates in this baseline were last source-verified on 2026-02-21 against SARS pages.",
            "Spousal section 4(q) estate duty deduction is modeled as unlimited in baseline assumptions.",
            "Use this baseline as a rule catalog; actual tax calculation must still apply deductions, liabilities, and valuation rules.",
            "Rates and thresholds should be versioned by tax year before production use.",
        ],
    }
}
