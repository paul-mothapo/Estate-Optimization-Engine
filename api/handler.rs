use crate::core::errors::EngineError;
use crate::core::domain::models::{EstateScenarioInput, ScenarioResult};
use crate::core::engine::optimizer::{optimize_scenarios, OptimizedScenario};
use crate::core::engine::scenario::calculate_combined_tax_and_liquidity;
use crate::core::validation::InputValidationError;

pub fn calculate_single_scenario(input: &EstateScenarioInput) -> Result<ScenarioResult, EngineError> {
    input.validate().map_err(EngineError::from)?;
    Ok(calculate_combined_tax_and_liquidity(input))
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
    Ok(optimize_scenarios(candidates))
}
