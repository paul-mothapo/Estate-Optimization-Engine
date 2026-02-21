use crate::core::models::{EstateScenarioInput, ScenarioResult};
use crate::core::optimizer::{optimize_scenarios, OptimizedScenario};
use crate::core::scenario::calculate_combined_tax_and_liquidity;

pub fn calculate_single_scenario(input: &EstateScenarioInput) -> ScenarioResult {
    calculate_combined_tax_and_liquidity(input)
}

pub fn optimize_candidate_scenarios(candidates: Vec<EstateScenarioInput>) -> Option<OptimizedScenario> {
    optimize_scenarios(candidates)
}
