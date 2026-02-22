use crate::api::contracts::{ApiErrorCode, ApiEstateAssetInput, ApiEstateScenarioInput};
use crate::api::handler::{
    calculate_single_scenario_contract, optimize_candidate_scenarios_contract,
};
use crate::core::domain::models::EstateScenarioInput;

fn valid_contract_input() -> ApiEstateScenarioInput {
    let mut input = ApiEstateScenarioInput::from(EstateScenarioInput::default());
    input.assets = vec![ApiEstateAssetInput {
        name: "Contract Asset".to_string(),
        market_value_zar: 1_000_000.0,
        base_cost_zar: 700_000.0,
        is_liquid: true,
        situs_in_south_africa: true,
        included_in_estate_duty: true,
        included_in_cgt_deemed_disposal: true,
        bequeathed_to_surviving_spouse: false,
        bequeathed_to_pbo: false,
        qualifies_primary_residence_exclusion: false,
    }];
    input.explicit_executor_fee_zar = Some(0.0);
    input
}

#[test]
fn contract_input_maps_to_domain_input() {
    let input = valid_contract_input();
    let domain: EstateScenarioInput = input.clone().into();

    assert_eq!(domain.jurisdiction, input.jurisdiction);
    assert_eq!(domain.tax_year, input.tax_year);
    assert_eq!(domain.assets.len(), 1);
    assert_eq!(domain.assets[0].name, "Contract Asset");
}

#[test]
fn calculate_single_scenario_contract_returns_api_result() {
    let input = valid_contract_input();
    let result = calculate_single_scenario_contract(input)
        .expect("Expected contract scenario calculation to succeed");

    assert!(result.combined_tax.total_tax_liability_zar >= 0.0);
    assert!(result.liquidity.total_available_liquidity_zar >= 0.0);
}

#[test]
fn calculate_single_scenario_contract_returns_api_validation_error() {
    let mut input = valid_contract_input();
    input.assets.clear();

    let err = calculate_single_scenario_contract(input)
        .expect_err("Expected contract scenario calculation to fail validation");
    assert_eq!(err.code, ApiErrorCode::Validation);
    assert!(err
        .validation_issues
        .iter()
        .any(|issue| issue.field == "assets"));
}

#[test]
fn optimize_candidate_scenarios_contract_returns_api_validation_error() {
    let valid = valid_contract_input();
    let mut invalid = valid_contract_input();
    invalid.assets[0].name = "".to_string();

    let err = optimize_candidate_scenarios_contract(vec![valid, invalid])
        .expect_err("Expected contract optimization to fail validation");
    assert_eq!(err.code, ApiErrorCode::Validation);
    assert!(err
        .validation_issues
        .iter()
        .any(|issue| issue.field == "candidates[1].assets[0].name"));
}
