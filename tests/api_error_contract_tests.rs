use crate::api::contracts::ApiErrorCode;
use crate::api::handler::{
    calculate_single_scenario_api, resolve_tax_rules_for_year_api, to_api_error_response,
};
use crate::core::domain::models::EstateScenarioInput;
use crate::core::errors::EngineError;
use crate::core::rules::tax_rules::Jurisdiction;
use crate::core::validation::{InputValidationError, ValidationIssue};

#[test]
fn api_scenario_wrapper_returns_validation_error_contract() {
    let input = EstateScenarioInput::default();

    let err = calculate_single_scenario_api(&input)
        .expect_err("Expected API wrapper to return validation error contract");

    assert_eq!(err.code, ApiErrorCode::Validation);
    assert!(!err.validation_issues.is_empty());
    assert!(err
        .validation_issues
        .iter()
        .any(|issue| issue.field == "assets"));
}

#[test]
fn api_rule_resolution_wrapper_returns_rule_selection_error_contract() {
    let err = resolve_tax_rules_for_year_api(Jurisdiction::SouthAfrica, 2017)
        .expect_err("Expected unsupported year to map to API error contract");

    assert_eq!(err.code, ApiErrorCode::RuleSelection);
    assert!(err.validation_issues.is_empty());
    assert!(err.message.contains("2017"));
}

#[test]
fn maps_computation_error_to_api_contract() {
    let err = to_api_error_response(EngineError::Computation("calculation overflow".to_string()));
    assert_eq!(err.code, ApiErrorCode::Computation);
    assert_eq!(err.message, "calculation overflow");
    assert!(err.validation_issues.is_empty());
}

#[test]
fn maps_validation_error_to_api_contract_with_issues() {
    let validation = InputValidationError::new(vec![ValidationIssue::new(
        "tax_year".to_string(),
        "Tax year is not supported",
    )]);
    let err = to_api_error_response(EngineError::Validation(validation));
    assert_eq!(err.code, ApiErrorCode::Validation);
    assert_eq!(err.validation_issues.len(), 1);
    assert_eq!(err.validation_issues[0].field, "tax_year");
}
