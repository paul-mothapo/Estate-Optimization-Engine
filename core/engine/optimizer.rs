use crate::core::domain::models::{EstateScenarioInput, ScenarioResult};
use crate::core::engine::scenario::calculate_combined_tax_and_liquidity;
use crate::core::engine::scoring::{score_scenario, ScenarioScore};

#[derive(Debug, Clone)]
pub struct OptimizedScenario {
    pub index: usize,
    pub input: EstateScenarioInput,
    pub result: ScenarioResult,
    pub score: ScenarioScore,
}

pub fn optimize_scenarios(candidates: Vec<EstateScenarioInput>) -> Option<OptimizedScenario> {
    candidates
        .into_iter()
        .enumerate()
        .map(|(index, input)| {
            let result = calculate_combined_tax_and_liquidity(&input);
            let score = score_scenario(&result);
            OptimizedScenario {
                index,
                input,
                result,
                score,
            }
        })
        .min_by(|a, b| a.score.composite_score.total_cmp(&b.score.composite_score))
}
