use crate::api::contracts::{
    ApiErrorCode, ApiErrorResponse, ApiValidationIssue, JurisdictionTaxRuleRegistryResponse,
};
use crate::core::errors::EngineError;
use crate::core::domain::models::{EstateScenarioInput, ScenarioResult};
use crate::core::engine::optimizer::{optimize_scenarios, OptimizedScenario};
use crate::core::engine::scenario::calculate_combined_tax_and_liquidity;
use crate::core::rules::tax_rules::{
    latest_tax_rules_for, supported_jurisdictions, supported_tax_year_window, tax_rule_registry,
    tax_rule_registry_for, tax_rules_for, Jurisdiction, TaxRuleRegistryEntry,
    VersionedJurisdictionTaxRuleSet,
};
use crate::core::validation::InputValidationError;

pub fn list_supported_jurisdictions() -> Vec<Jurisdiction> {
    supported_jurisdictions()
}

pub fn list_tax_rule_registry_entries() -> Vec<TaxRuleRegistryEntry> {
    tax_rule_registry()
}

pub fn get_jurisdiction_tax_rule_registry(
    jurisdiction: Jurisdiction,
) -> Option<JurisdictionTaxRuleRegistryResponse> {
    let versions = tax_rule_registry_for(jurisdiction);
    if versions.is_empty() {
        return None;
    }

    let (supported_tax_year_from, supported_tax_year_to) = supported_tax_year_window(jurisdiction)?;
    let latest_version_id = latest_tax_rules_for(jurisdiction).version.version_id;

    Some(JurisdictionTaxRuleRegistryResponse {
        jurisdiction,
        versions,
        supported_tax_year_from,
        supported_tax_year_to,
        latest_version_id,
    })
}

pub fn resolve_tax_rules_for_year(
    jurisdiction: Jurisdiction,
    tax_year: u16,
) -> Result<VersionedJurisdictionTaxRuleSet, EngineError> {
    tax_rules_for(jurisdiction, tax_year).map_err(EngineError::from)
}

pub fn resolve_latest_tax_rules(jurisdiction: Jurisdiction) -> VersionedJurisdictionTaxRuleSet {
    latest_tax_rules_for(jurisdiction)
}

pub fn to_api_error_response(error: EngineError) -> ApiErrorResponse {
    match error {
        EngineError::Validation(validation_error) => ApiErrorResponse {
            code: ApiErrorCode::Validation,
            message: validation_error.to_string(),
            validation_issues: validation_error
                .issues
                .into_iter()
                .map(|issue| ApiValidationIssue {
                    field: issue.field,
                    message: issue.message,
                })
                .collect(),
        },
        EngineError::RuleSelection(selection_error) => ApiErrorResponse {
            code: ApiErrorCode::RuleSelection,
            message: selection_error.to_string(),
            validation_issues: Vec::new(),
        },
        EngineError::Computation(message) => ApiErrorResponse {
            code: ApiErrorCode::Computation,
            message,
            validation_issues: Vec::new(),
        },
    }
}

pub fn calculate_single_scenario_api(
    input: &EstateScenarioInput,
) -> Result<ScenarioResult, ApiErrorResponse> {
    calculate_single_scenario(input).map_err(to_api_error_response)
}

pub fn optimize_candidate_scenarios_api(
    candidates: Vec<EstateScenarioInput>,
) -> Result<Option<OptimizedScenario>, ApiErrorResponse> {
    optimize_candidate_scenarios(candidates).map_err(to_api_error_response)
}

pub fn resolve_tax_rules_for_year_api(
    jurisdiction: Jurisdiction,
    tax_year: u16,
) -> Result<VersionedJurisdictionTaxRuleSet, ApiErrorResponse> {
    resolve_tax_rules_for_year(jurisdiction, tax_year).map_err(to_api_error_response)
}

pub fn calculate_single_scenario(input: &EstateScenarioInput) -> Result<ScenarioResult, EngineError> {
    input.validate().map_err(EngineError::from)?;
    calculate_combined_tax_and_liquidity(input).map_err(EngineError::from)
}

pub fn optimize_candidate_scenarios(
    candidates: Vec<EstateScenarioInput>,
) -> Result<Option<OptimizedScenario>, EngineError> {
    let mut all_issues = Vec::new();
    for (index, candidate) in candidates.iter().enumerate() {
        if let Err(err) = candidate.validate() {
            for mut issue in err.issues {
                issue.field = format!("candidates[{index}].{}", issue.field);
                all_issues.push(issue);
            }
        }
    }
    if !all_issues.is_empty() {
        return Err(EngineError::Validation(InputValidationError::new(all_issues)));
    }
    optimize_scenarios(candidates).map_err(EngineError::from)
}
