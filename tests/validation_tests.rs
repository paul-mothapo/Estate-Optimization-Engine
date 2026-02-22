use crate::api::handler::{calculate_single_scenario, optimize_candidate_scenarios};
use crate::core::errors::EngineError;
use crate::core::domain::models::{EstateAsset, EstateScenarioInput, ResidencyStatus};
use crate::core::rules::tax_rules::{Jurisdiction, TaxPayerClass};

fn valid_input() -> EstateScenarioInput {
    EstateScenarioInput {
        jurisdiction: Jurisdiction::SouthAfrica,
        taxpayer_class: TaxPayerClass::NaturalPerson,
        residency_status: ResidencyStatus::Resident,
        marginal_income_tax_rate: 0.45,
        assets: vec![EstateAsset {
            name: "Test Asset".to_string(),
            market_value_zar: 1_000_000.0,
            base_cost_zar: 700_000.0,
            is_liquid: true,
            situs_in_south_africa: true,
            included_in_estate_duty: true,
            included_in_cgt_deemed_disposal: true,
            bequeathed_to_surviving_spouse: false,
            bequeathed_to_pbo: false,
            qualifies_primary_residence_exclusion: false,
        }],
        explicit_executor_fee_zar: Some(0.0),
        ..EstateScenarioInput::default()
    }
}

#[test]
fn rejects_empty_assets() {
    let mut input = valid_input();
    input.assets.clear();

    let err = input.validate().expect_err("Expected validation to fail");
    assert!(!err.is_empty());
    assert!(err.issues.iter().any(|i| i.field == "assets"));
}

#[test]
fn rejects_out_of_range_rates() {
    let mut input = valid_input();
    input.marginal_income_tax_rate = 1.5;
    input.vat_rate = -0.1;

    let err = input.validate().expect_err("Expected validation to fail");
    assert!(err
        .issues
        .iter()
        .any(|i| i.field == "marginal_income_tax_rate"));
    assert!(err.issues.iter().any(|i| i.field == "vat_rate"));
}

#[test]
fn rejects_negative_amounts() {
    let mut input = valid_input();
    input.funeral_costs_zar = -10.0;
    input.assets[0].market_value_zar = -100.0;

    let err = input.validate().expect_err("Expected validation to fail");
    assert!(err.issues.iter().any(|i| i.field == "funeral_costs_zar"));
    assert!(err
        .issues
        .iter()
        .any(|i| i.field == "assets[0].market_value_zar"));
}

#[test]
fn api_calculate_rejects_invalid_input() {
    let mut input = valid_input();
    input.executor_fee_rate = 2.0;

    let err = calculate_single_scenario(&input).expect_err("Expected API validation error");
    let EngineError::Validation(err) = err else {
        panic!("Expected validation error");
    };
    assert!(err.issues.iter().any(|i| i.field == "executor_fee_rate"));
}

#[test]
fn api_optimize_rejects_invalid_candidate() {
    let valid = valid_input();
    let mut invalid = valid_input();
    invalid.assets[0].name = "".to_string();

    let err = optimize_candidate_scenarios(vec![valid, invalid])
        .expect_err("Expected API validation error");
    let EngineError::Validation(err) = err else {
        panic!("Expected validation error");
    };
    assert!(err
        .issues
        .iter()
        .any(|i| i.field == "candidates[1].assets[0].name"));
}

#[test]
fn rejects_spouse_or_pbo_bequest_when_not_in_estate_duty_scope() {
    let mut input = valid_input();
    input.assets[0].included_in_estate_duty = false;
    input.assets[0].bequeathed_to_surviving_spouse = true;

    let err = input.validate().expect_err("Expected validation to fail");
    assert!(err
        .issues
        .iter()
        .any(|i| i.field == "assets[0].included_in_estate_duty"));
}

#[test]
fn rejects_primary_residence_flag_without_cgt_inclusion() {
    let mut input = valid_input();
    input.assets[0].qualifies_primary_residence_exclusion = true;
    input.assets[0].included_in_cgt_deemed_disposal = false;

    let err = input.validate().expect_err("Expected validation to fail");
    assert!(err
        .issues
        .iter()
        .any(|i| i.field == "assets[0].included_in_cgt_deemed_disposal"));
}

#[test]
fn rejects_primary_residence_exclusion_for_company() {
    let mut input = valid_input();
    input.taxpayer_class = TaxPayerClass::Company;
    input.primary_residence_cgt_exclusion_cap_zar = 2_000_000.0;
    input.assets[0].qualifies_primary_residence_exclusion = true;

    let err = input.validate().expect_err("Expected validation to fail");
    assert!(err
        .issues
        .iter()
        .any(|i| i.field == "primary_residence_cgt_exclusion_cap_zar"));
    assert!(err
        .issues
        .iter()
        .any(|i| i.field == "assets[0].qualifies_primary_residence_exclusion"));
}

#[test]
fn rejects_non_resident_foreign_asset_marked_for_estate_duty() {
    let mut input = valid_input();
    input.residency_status = ResidencyStatus::NonResident;
    input.assets[0].situs_in_south_africa = false;
    input.assets[0].included_in_estate_duty = true;

    let err = input.validate().expect_err("Expected validation to fail");
    assert!(err
        .issues
        .iter()
        .any(|i| i.field == "assets[0].situs_in_south_africa"));
}
