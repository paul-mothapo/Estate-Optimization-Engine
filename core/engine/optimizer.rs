use crate::core::domain::models::{EstateScenarioInput, ScenarioResult};
use crate::core::engine::scenario::calculate_combined_tax_and_liquidity;
use crate::core::engine::scoring::{score_scenario, ScenarioScore};
use crate::core::rules::tax_rules::TaxRuleSelectionError;

#[derive(Debug, Clone)]
pub struct OptimizedScenario {
    pub index: usize,
    pub input: EstateScenarioInput,
    pub result: ScenarioResult,
    pub score: ScenarioScore,
}

pub fn optimize_scenarios(
    candidates: Vec<EstateScenarioInput>,
) -> Result<Option<OptimizedScenario>, TaxRuleSelectionError> {
    let mut optimized = Vec::new();

    for (index, input) in candidates.into_iter().enumerate() {
        let result = calculate_combined_tax_and_liquidity(&input)?;
        let score = score_scenario(&result);
        optimized.push(OptimizedScenario {
            index,
            input,
            result,
            score,
        });
    }

    Ok(optimized
        .into_iter()
        .min_by(|a, b| a.score.composite_score.total_cmp(&b.score.composite_score)))
}
