use crate::core::domain::models::{EstateAsset, EstateScenarioInput, ResidencyStatus};
use crate::core::engine::scenario::calculate_combined_tax_and_liquidity;
use crate::core::rules::tax_rules::{Jurisdiction, TaxPayerClass, TaxRuleSelectionError};

fn baseline_input() -> EstateScenarioInput {
    EstateScenarioInput {
        jurisdiction: Jurisdiction::SouthAfrica,
        taxpayer_class: TaxPayerClass::NaturalPerson,
        residency_status: ResidencyStatus::Resident,
        marginal_income_tax_rate: 0.45,
        explicit_executor_fee_amount: Some(0.0),
        ..EstateScenarioInput::default()
    }
}

#[test]
fn applies_estate_duty_primary_and_secondary_bands() {
    let mut input = baseline_input();
    input.assets.push(EstateAsset {
        name: "Illiquid asset".to_string(),
        market_value_amount: 40_000_000.0,
        base_cost_amount: 40_000_000.0,
        is_liquid: false,
        situs_in_jurisdiction: true,
        included_in_estate_duty: true,
        included_in_cgt_deemed_disposal: false,
        bequeathed_to_surviving_spouse: false,
        bequeathed_to_pbo: false,
        qualifies_primary_residence_exclusion: false,
    });

    let result = calculate_combined_tax_and_liquidity(&input)
        .expect("Expected calculation to succeed for supported tax year");
    assert!((result.estate_duty.tax_payable_amount - 7_625_000.0).abs() < 0.1);
    assert!((result.liquidity.liquidity_gap_amount - 7_625_000.0).abs() < 0.1);
}

#[test]
fn applies_section_4q_spousal_deduction() {
    let mut input = baseline_input();
    input.assets.extend([
        EstateAsset {
            name: "Spouse bequest".to_string(),
            market_value_amount: 20_000_000.0,
            base_cost_amount: 20_000_000.0,
            is_liquid: false,
            situs_in_jurisdiction: true,
            included_in_estate_duty: true,
            included_in_cgt_deemed_disposal: false,
            bequeathed_to_surviving_spouse: true,
            bequeathed_to_pbo: false,
            qualifies_primary_residence_exclusion: false,
        },
        EstateAsset {
            name: "Non-spouse bequest".to_string(),
            market_value_amount: 5_000_000.0,
            base_cost_amount: 5_000_000.0,
            is_liquid: false,
            situs_in_jurisdiction: true,
            included_in_estate_duty: true,
            included_in_cgt_deemed_disposal: false,
            bequeathed_to_surviving_spouse: false,
            bequeathed_to_pbo: false,
            qualifies_primary_residence_exclusion: false,
        },
    ]);

    let result = calculate_combined_tax_and_liquidity(&input)
        .expect("Expected calculation to succeed for supported tax year");
    assert!((result.estate_duty.spousal_deduction_amount - 20_000_000.0).abs() < 0.1);
    assert!((result.estate_duty.tax_payable_amount - 300_000.0).abs() < 0.1);
}

#[test]
fn excludes_foreign_assets_for_non_resident_estate_duty_scope() {
    let mut input = baseline_input();
    input.residency_status = ResidencyStatus::NonResident;
    input.assets.extend([
        EstateAsset {
            name: "SA situs asset".to_string(),
            market_value_amount: 10_000_000.0,
            base_cost_amount: 10_000_000.0,
            is_liquid: false,
            situs_in_jurisdiction: true,
            included_in_estate_duty: true,
            included_in_cgt_deemed_disposal: false,
            bequeathed_to_surviving_spouse: false,
            bequeathed_to_pbo: false,
            qualifies_primary_residence_exclusion: false,
        },
        EstateAsset {
            name: "Foreign situs asset".to_string(),
            market_value_amount: 50_000_000.0,
            base_cost_amount: 50_000_000.0,
            is_liquid: false,
            situs_in_jurisdiction: false,
            included_in_estate_duty: true,
            included_in_cgt_deemed_disposal: false,
            bequeathed_to_surviving_spouse: false,
            bequeathed_to_pbo: false,
            qualifies_primary_residence_exclusion: false,
        },
    ]);

    let result = calculate_combined_tax_and_liquidity(&input)
        .expect("Expected calculation to succeed for supported tax year");
    assert!((result.estate_duty.gross_estate_for_transfer_tax_amount - 10_000_000.0).abs() < 0.1);
}

#[test]
fn calculation_rejects_unsupported_tax_year() {
    let mut input = baseline_input();
    input.tax_year = 2017;

    let err = calculate_combined_tax_and_liquidity(&input)
        .expect_err("Expected unsupported tax year to fail rule selection");
    assert_eq!(
        err,
        TaxRuleSelectionError::UnsupportedTaxYear {
            jurisdiction: Jurisdiction::SouthAfrica,
            tax_year: 2017,
        }
    );
}

#[test]
fn us_state_baseline_applies_step_up_and_no_cgt_at_death() {
    let mut input = baseline_input();
    input.jurisdiction = Jurisdiction::UsCalifornia;
    input.tax_year = 2026;
    input.marginal_income_tax_rate = 0.37;
    input.primary_residence_cgt_exclusion_cap_amount = 0.0;
    input.assets.push(EstateAsset {
        name: "US appreciated asset".to_string(),
        market_value_amount: 9_000_000.0,
        base_cost_amount: 1_000_000.0,
        is_liquid: false,
        situs_in_jurisdiction: true,
        included_in_estate_duty: true,
        included_in_cgt_deemed_disposal: true,
        bequeathed_to_surviving_spouse: false,
        bequeathed_to_pbo: false,
        qualifies_primary_residence_exclusion: false,
    });

    let result = calculate_combined_tax_and_liquidity(&input)
        .expect("Expected US state baseline calculation to succeed");
    assert_eq!(result.cgt.tax_payable_amount, 0.0);
    assert_eq!(result.cgt.taxable_capital_gain_in_income_amount, 0.0);
}

#[test]
fn us_state_baseline_rejects_pre_2026_tax_year() {
    let mut input = baseline_input();
    input.jurisdiction = Jurisdiction::UsTexas;
    input.tax_year = 2025;

    let err = calculate_combined_tax_and_liquidity(&input)
        .expect_err("Expected pre-2026 year to fail US state rule selection");
    assert_eq!(
        err,
        TaxRuleSelectionError::UnsupportedTaxYear {
            jurisdiction: Jurisdiction::UsTexas,
            tax_year: 2025,
        }
    );
}
