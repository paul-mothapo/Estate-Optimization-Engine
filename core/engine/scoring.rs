use crate::core::domain::models::ScenarioResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiquidityRiskBand {
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct ScenarioScore {
    pub tax_burden_ratio: f64,
    pub liquidity_cover_ratio: f64,
    pub liquidity_risk_band: LiquidityRiskBand,
    pub composite_score: f64,
}

pub fn score_scenario(result: &ScenarioResult) -> ScenarioScore {
    let gross_estate = result.estate_duty.gross_estate_for_estate_duty_zar.max(1.0);
    let required_liquidity = result.liquidity.immediate_cash_requirements_zar.max(1.0);

    let tax_burden_ratio = result.combined_tax.total_tax_liability_zar / gross_estate;
    let liquidity_cover_ratio = result.liquidity.total_available_liquidity_zar / required_liquidity;

    let liquidity_risk_band = if liquidity_cover_ratio >= 1.20 {
        LiquidityRiskBand::Low
    } else if liquidity_cover_ratio >= 1.0 {
        LiquidityRiskBand::Moderate
    } else if liquidity_cover_ratio >= 0.80 {
        LiquidityRiskBand::High
    } else {
        LiquidityRiskBand::Critical
    };

    let tax_penalty = tax_burden_ratio * 100.0;
    let liquidity_penalty = if liquidity_cover_ratio >= 1.0 {
        0.0
    } else {
        (1.0 - liquidity_cover_ratio) * 200.0
    };

    ScenarioScore {
        tax_burden_ratio,
        liquidity_cover_ratio,
        liquidity_risk_band,
        composite_score: tax_penalty + liquidity_penalty,
    }
}
