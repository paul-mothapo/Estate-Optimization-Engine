use crate::core::domain::models::{EstateScenarioInput, ScenarioResult};
use crate::core::engine::scenario::calculate_combined_tax_and_liquidity;
use crate::core::rules::tax_rules::TaxRuleSelectionError;

#[derive(Debug, Clone)]
pub struct StressResult {
    pub market_value_shock: f64,
    pub liquid_asset_haircut: f64,
    pub outcome: ScenarioResult,
}

pub fn run_liquidity_stress_grid(
    base_input: &EstateScenarioInput,
    market_value_shocks: &[f64],
    liquid_asset_haircuts: &[f64],
) -> Result<Vec<StressResult>, TaxRuleSelectionError> {
    let mut results = Vec::new();

    for shock in market_value_shocks {
        for haircut in liquid_asset_haircuts {
            let mut shocked_input = base_input.clone();
            let applied_shock = (1.0 + shock).max(0.0);
            let applied_liquid_haircut = (1.0 - haircut).clamp(0.0, 1.0);

            for asset in &mut shocked_input.assets {
                asset.market_value_zar = (asset.market_value_zar * applied_shock).max(0.0);
                if asset.is_liquid {
                    asset.market_value_zar *= applied_liquid_haircut;
                }
            }

            let outcome = calculate_combined_tax_and_liquidity(&shocked_input)?;
            results.push(StressResult {
                market_value_shock: *shock,
                liquid_asset_haircut: *haircut,
                outcome,
            });
        }
    }

    Ok(results)
}
