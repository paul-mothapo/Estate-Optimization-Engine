use crate::core::errors::EngineError;
use crate::core::domain::models::{EstateScenarioInput, ScenarioResult};
use crate::core::engine::optimizer::{optimize_scenarios, OptimizedScenario};
use crate::core::engine::scenario::calculate_combined_tax_and_liquidity;
use crate::core::rules::tax_rules::{
    latest_tax_rules_for, supported_jurisdictions, supported_tax_year_window, tax_rule_registry,
    tax_rule_registry_for, Jurisdiction, TaxRuleRegistryEntry, TaxRuleVersion,
};
use crate::core::validation::InputValidationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JurisdictionTaxRuleRegistryResponse {
    pub jurisdiction: Jurisdiction,
    pub versions: Vec<TaxRuleVersion>,
    pub supported_tax_year_from: u16,
    pub supported_tax_year_to: Option<u16>,
    pub latest_version_id: &'static str,
}

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
