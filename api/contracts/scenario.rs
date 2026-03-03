use super::ApiJurisdiction;
use crate::core::domain::models::{
    CapitalGainsTaxBreakdown, CombinedTaxLiability, EstateAsset, EstateDutyBreakdown,
    EstateScenarioInput, LiquidityGapOutput, ResidencyStatus, ScenarioResult,
};
use crate::core::engine::optimizer::OptimizedScenario;
use crate::core::engine::scoring::{LiquidityRiskBand, ScenarioScore};
use crate::core::rules::tax_rules::TaxPayerClass;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiEstateAssetInput {
    pub name: String,
    #[serde(alias = "market_value_zar")]
    pub market_value_amount: f64,
    #[serde(alias = "base_cost_zar")]
    pub base_cost_amount: f64,
    pub is_liquid: bool,
    #[serde(alias = "situs_in_south_africa")]
    pub situs_in_jurisdiction: bool,
    pub included_in_estate_duty: bool,
    pub included_in_cgt_deemed_disposal: bool,
    pub bequeathed_to_surviving_spouse: bool,
    pub bequeathed_to_pbo: bool,
    pub qualifies_primary_residence_exclusion: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum ApiTaxPayerClass {
    NaturalPerson,
    Company,
    Trust,
    SpecialTrust,
}

impl From<TaxPayerClass> for ApiTaxPayerClass {
    fn from(value: TaxPayerClass) -> Self {
        match value {
            TaxPayerClass::NaturalPerson => ApiTaxPayerClass::NaturalPerson,
            TaxPayerClass::Company => ApiTaxPayerClass::Company,
            TaxPayerClass::Trust => ApiTaxPayerClass::Trust,
            TaxPayerClass::SpecialTrust => ApiTaxPayerClass::SpecialTrust,
        }
    }
}

impl From<ApiTaxPayerClass> for TaxPayerClass {
    fn from(value: ApiTaxPayerClass) -> Self {
        match value {
            ApiTaxPayerClass::NaturalPerson => TaxPayerClass::NaturalPerson,
            ApiTaxPayerClass::Company => TaxPayerClass::Company,
            ApiTaxPayerClass::Trust => TaxPayerClass::Trust,
            ApiTaxPayerClass::SpecialTrust => TaxPayerClass::SpecialTrust,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum ApiResidencyStatus {
    Resident,
    NonResident,
}

impl From<ResidencyStatus> for ApiResidencyStatus {
    fn from(value: ResidencyStatus) -> Self {
        match value {
            ResidencyStatus::Resident => ApiResidencyStatus::Resident,
            ResidencyStatus::NonResident => ApiResidencyStatus::NonResident,
        }
    }
}

impl From<ApiResidencyStatus> for ResidencyStatus {
    fn from(value: ApiResidencyStatus) -> Self {
        match value {
            ApiResidencyStatus::Resident => ResidencyStatus::Resident,
            ApiResidencyStatus::NonResident => ResidencyStatus::NonResident,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiEstateScenarioInput {
    pub jurisdiction: ApiJurisdiction,
    pub tax_year: u16,
    pub taxpayer_class: ApiTaxPayerClass,
    pub residency_status: ApiResidencyStatus,
    pub marginal_income_tax_rate: f64,
    pub assets: Vec<ApiEstateAssetInput>,
    #[serde(alias = "debts_and_loans_zar")]
    pub debts_and_loans_amount: f64,
    #[serde(alias = "funeral_costs_zar")]
    pub funeral_costs_amount: f64,
    #[serde(alias = "administration_costs_zar")]
    pub administration_costs_amount: f64,
    #[serde(alias = "masters_office_fees_zar")]
    pub masters_office_fees_amount: f64,
    #[serde(alias = "conveyancing_costs_zar")]
    pub conveyancing_costs_amount: f64,
    #[serde(alias = "other_settlement_costs_zar")]
    pub other_settlement_costs_amount: f64,
    #[serde(alias = "final_income_tax_due_zar")]
    pub final_income_tax_due_amount: f64,
    #[serde(alias = "ongoing_estate_income_tax_provision_zar")]
    pub ongoing_estate_income_tax_provision_amount: f64,
    #[serde(alias = "additional_allowable_estate_duty_deductions_zar")]
    pub additional_allowable_estate_transfer_tax_deductions_amount: f64,
    #[serde(alias = "ported_section_4a_abatement_zar")]
    pub ported_estate_tax_exemption_amount: f64,
    #[serde(alias = "primary_residence_cgt_exclusion_cap_zar")]
    pub primary_residence_cgt_exclusion_cap_amount: f64,
    pub executor_fee_rate: f64,
    pub vat_rate: f64,
    #[serde(alias = "explicit_executor_fee_zar")]
    pub explicit_executor_fee_amount: Option<f64>,
    #[serde(alias = "external_liquidity_proceeds_zar")]
    pub external_liquidity_proceeds_amount: f64,
    #[serde(alias = "cash_reserve_zar")]
    pub cash_reserve_amount: f64,
}

impl From<ApiEstateAssetInput> for EstateAsset {
    fn from(value: ApiEstateAssetInput) -> Self {
        EstateAsset {
            name: value.name,
            market_value_amount: value.market_value_amount,
            base_cost_amount: value.base_cost_amount,
            is_liquid: value.is_liquid,
            situs_in_jurisdiction: value.situs_in_jurisdiction,
            included_in_estate_duty: value.included_in_estate_duty,
            included_in_cgt_deemed_disposal: value.included_in_cgt_deemed_disposal,
            bequeathed_to_surviving_spouse: value.bequeathed_to_surviving_spouse,
            bequeathed_to_pbo: value.bequeathed_to_pbo,
            qualifies_primary_residence_exclusion: value.qualifies_primary_residence_exclusion,
        }
    }
}

impl From<EstateAsset> for ApiEstateAssetInput {
    fn from(value: EstateAsset) -> Self {
        ApiEstateAssetInput {
            name: value.name,
            market_value_amount: value.market_value_amount,
            base_cost_amount: value.base_cost_amount,
            is_liquid: value.is_liquid,
            situs_in_jurisdiction: value.situs_in_jurisdiction,
            included_in_estate_duty: value.included_in_estate_duty,
            included_in_cgt_deemed_disposal: value.included_in_cgt_deemed_disposal,
            bequeathed_to_surviving_spouse: value.bequeathed_to_surviving_spouse,
            bequeathed_to_pbo: value.bequeathed_to_pbo,
            qualifies_primary_residence_exclusion: value.qualifies_primary_residence_exclusion,
        }
    }
}

impl From<ApiEstateScenarioInput> for EstateScenarioInput {
    fn from(value: ApiEstateScenarioInput) -> Self {
        EstateScenarioInput {
            jurisdiction: value.jurisdiction.into(),
            tax_year: value.tax_year,
            taxpayer_class: value.taxpayer_class.into(),
            residency_status: value.residency_status.into(),
            marginal_income_tax_rate: value.marginal_income_tax_rate,
            assets: value.assets.into_iter().map(EstateAsset::from).collect(),
            debts_and_loans_amount: value.debts_and_loans_amount,
            funeral_costs_amount: value.funeral_costs_amount,
            administration_costs_amount: value.administration_costs_amount,
            masters_office_fees_amount: value.masters_office_fees_amount,
            conveyancing_costs_amount: value.conveyancing_costs_amount,
            other_settlement_costs_amount: value.other_settlement_costs_amount,
            final_income_tax_due_amount: value.final_income_tax_due_amount,
            ongoing_estate_income_tax_provision_amount: value
                .ongoing_estate_income_tax_provision_amount,
            additional_allowable_estate_transfer_tax_deductions_amount: value
                .additional_allowable_estate_transfer_tax_deductions_amount,
            ported_estate_tax_exemption_amount: value.ported_estate_tax_exemption_amount,
            primary_residence_cgt_exclusion_cap_amount: value
                .primary_residence_cgt_exclusion_cap_amount,
            executor_fee_rate: value.executor_fee_rate,
            vat_rate: value.vat_rate,
            explicit_executor_fee_amount: value.explicit_executor_fee_amount,
            external_liquidity_proceeds_amount: value.external_liquidity_proceeds_amount,
            cash_reserve_amount: value.cash_reserve_amount,
        }
    }
}

impl From<EstateScenarioInput> for ApiEstateScenarioInput {
    fn from(value: EstateScenarioInput) -> Self {
        ApiEstateScenarioInput {
            jurisdiction: value.jurisdiction.into(),
            tax_year: value.tax_year,
            taxpayer_class: value.taxpayer_class.into(),
            residency_status: value.residency_status.into(),
            marginal_income_tax_rate: value.marginal_income_tax_rate,
            assets: value
                .assets
                .into_iter()
                .map(ApiEstateAssetInput::from)
                .collect(),
            debts_and_loans_amount: value.debts_and_loans_amount,
            funeral_costs_amount: value.funeral_costs_amount,
            administration_costs_amount: value.administration_costs_amount,
            masters_office_fees_amount: value.masters_office_fees_amount,
            conveyancing_costs_amount: value.conveyancing_costs_amount,
            other_settlement_costs_amount: value.other_settlement_costs_amount,
            final_income_tax_due_amount: value.final_income_tax_due_amount,
            ongoing_estate_income_tax_provision_amount: value
                .ongoing_estate_income_tax_provision_amount,
            additional_allowable_estate_transfer_tax_deductions_amount: value
                .additional_allowable_estate_transfer_tax_deductions_amount,
            ported_estate_tax_exemption_amount: value.ported_estate_tax_exemption_amount,
            primary_residence_cgt_exclusion_cap_amount: value
                .primary_residence_cgt_exclusion_cap_amount,
            executor_fee_rate: value.executor_fee_rate,
            vat_rate: value.vat_rate,
            explicit_executor_fee_amount: value.explicit_executor_fee_amount,
            external_liquidity_proceeds_amount: value.external_liquidity_proceeds_amount,
            cash_reserve_amount: value.cash_reserve_amount,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiCapitalGainsTaxBreakdown {
    pub gross_capital_gain_amount: f64,
    pub primary_residence_exclusion_used_amount: f64,
    pub annual_exclusion_used_amount: f64,
    pub inclusion_rate: f64,
    pub taxable_capital_gain_in_income_amount: f64,
    pub tax_payable_amount: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiEstateDutyBreakdown {
    pub gross_estate_for_transfer_tax_amount: f64,
    pub executor_fee_amount: f64,
    pub spousal_deduction_amount: f64,
    pub pbo_deduction_amount: f64,
    pub total_allowable_deductions_amount: f64,
    pub exemption_used_amount: f64,
    pub dutiable_estate_after_exemption_amount: f64,
    pub tax_payable_amount: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiCombinedTaxLiability {
    pub estate_transfer_tax_amount: f64,
    pub cgt_on_death_amount: f64,
    pub final_income_tax_amount: f64,
    pub ongoing_estate_income_tax_provision_amount: f64,
    pub total_tax_liability_amount: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiLiquidityGapOutput {
    pub liquid_assets_in_estate_amount: f64,
    pub external_liquidity_proceeds_amount: f64,
    pub cash_reserve_amount: f64,
    pub total_available_liquidity_amount: f64,
    pub executor_fee_amount: f64,
    pub immediate_cash_requirements_amount: f64,
    pub liquidity_gap_amount: f64,
    pub liquidity_surplus_amount: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiScenarioResult {
    pub cgt: ApiCapitalGainsTaxBreakdown,
    pub estate_duty: ApiEstateDutyBreakdown,
    pub combined_tax: ApiCombinedTaxLiability,
    pub liquidity: ApiLiquidityGapOutput,
}

impl From<CapitalGainsTaxBreakdown> for ApiCapitalGainsTaxBreakdown {
    fn from(value: CapitalGainsTaxBreakdown) -> Self {
        ApiCapitalGainsTaxBreakdown {
            gross_capital_gain_amount: value.gross_capital_gain_amount,
            primary_residence_exclusion_used_amount: value.primary_residence_exclusion_used_amount,
            annual_exclusion_used_amount: value.annual_exclusion_used_amount,
            inclusion_rate: value.inclusion_rate,
            taxable_capital_gain_in_income_amount: value.taxable_capital_gain_in_income_amount,
            tax_payable_amount: value.tax_payable_amount,
        }
    }
}

impl From<EstateDutyBreakdown> for ApiEstateDutyBreakdown {
    fn from(value: EstateDutyBreakdown) -> Self {
        ApiEstateDutyBreakdown {
            gross_estate_for_transfer_tax_amount: value.gross_estate_for_transfer_tax_amount,
            executor_fee_amount: value.executor_fee_amount,
            spousal_deduction_amount: value.spousal_deduction_amount,
            pbo_deduction_amount: value.pbo_deduction_amount,
            total_allowable_deductions_amount: value.total_allowable_deductions_amount,
            exemption_used_amount: value.exemption_used_amount,
            dutiable_estate_after_exemption_amount: value.dutiable_estate_after_exemption_amount,
            tax_payable_amount: value.tax_payable_amount,
        }
    }
}

impl From<CombinedTaxLiability> for ApiCombinedTaxLiability {
    fn from(value: CombinedTaxLiability) -> Self {
        ApiCombinedTaxLiability {
            estate_transfer_tax_amount: value.estate_transfer_tax_amount,
            cgt_on_death_amount: value.cgt_on_death_amount,
            final_income_tax_amount: value.final_income_tax_amount,
            ongoing_estate_income_tax_provision_amount: value
                .ongoing_estate_income_tax_provision_amount,
            total_tax_liability_amount: value.total_tax_liability_amount,
        }
    }
}

impl From<LiquidityGapOutput> for ApiLiquidityGapOutput {
    fn from(value: LiquidityGapOutput) -> Self {
        ApiLiquidityGapOutput {
            liquid_assets_in_estate_amount: value.liquid_assets_in_estate_amount,
            external_liquidity_proceeds_amount: value.external_liquidity_proceeds_amount,
            cash_reserve_amount: value.cash_reserve_amount,
            total_available_liquidity_amount: value.total_available_liquidity_amount,
            executor_fee_amount: value.executor_fee_amount,
            immediate_cash_requirements_amount: value.immediate_cash_requirements_amount,
            liquidity_gap_amount: value.liquidity_gap_amount,
            liquidity_surplus_amount: value.liquidity_surplus_amount,
        }
    }
}

impl From<ScenarioResult> for ApiScenarioResult {
    fn from(value: ScenarioResult) -> Self {
        ApiScenarioResult {
            cgt: value.cgt.into(),
            estate_duty: value.estate_duty.into(),
            combined_tax: value.combined_tax.into(),
            liquidity: value.liquidity.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ApiLiquidityRiskBand {
    Low,
    Moderate,
    High,
    Critical,
}

impl From<LiquidityRiskBand> for ApiLiquidityRiskBand {
    fn from(value: LiquidityRiskBand) -> Self {
        match value {
            LiquidityRiskBand::Low => ApiLiquidityRiskBand::Low,
            LiquidityRiskBand::Moderate => ApiLiquidityRiskBand::Moderate,
            LiquidityRiskBand::High => ApiLiquidityRiskBand::High,
            LiquidityRiskBand::Critical => ApiLiquidityRiskBand::Critical,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiScenarioScore {
    pub tax_burden_ratio: f64,
    pub liquidity_cover_ratio: f64,
    pub liquidity_risk_band: ApiLiquidityRiskBand,
    pub composite_score: f64,
}

impl From<ScenarioScore> for ApiScenarioScore {
    fn from(value: ScenarioScore) -> Self {
        ApiScenarioScore {
            tax_burden_ratio: value.tax_burden_ratio,
            liquidity_cover_ratio: value.liquidity_cover_ratio,
            liquidity_risk_band: value.liquidity_risk_band.into(),
            composite_score: value.composite_score,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiOptimizedScenario {
    pub index: usize,
    pub input: ApiEstateScenarioInput,
    pub result: ApiScenarioResult,
    pub score: ApiScenarioScore,
}

impl From<OptimizedScenario> for ApiOptimizedScenario {
    fn from(value: OptimizedScenario) -> Self {
        ApiOptimizedScenario {
            index: value.index,
            input: value.input.into(),
            result: value.result.into(),
            score: value.score.into(),
        }
    }
}
