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
    pub market_value_zar: f64,
    pub base_cost_zar: f64,
    pub is_liquid: bool,
    pub situs_in_south_africa: bool,
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
    pub debts_and_loans_zar: f64,
    pub funeral_costs_zar: f64,
    pub administration_costs_zar: f64,
    pub masters_office_fees_zar: f64,
    pub conveyancing_costs_zar: f64,
    pub other_settlement_costs_zar: f64,
    pub final_income_tax_due_zar: f64,
    pub ongoing_estate_income_tax_provision_zar: f64,
    pub additional_allowable_estate_duty_deductions_zar: f64,
    pub ported_section_4a_abatement_zar: f64,
    pub primary_residence_cgt_exclusion_cap_zar: f64,
    pub executor_fee_rate: f64,
    pub vat_rate: f64,
    pub explicit_executor_fee_zar: Option<f64>,
    pub external_liquidity_proceeds_zar: f64,
    pub cash_reserve_zar: f64,
}

impl From<ApiEstateAssetInput> for EstateAsset {
    fn from(value: ApiEstateAssetInput) -> Self {
        EstateAsset {
            name: value.name,
            market_value_zar: value.market_value_zar,
            base_cost_zar: value.base_cost_zar,
            is_liquid: value.is_liquid,
            situs_in_south_africa: value.situs_in_south_africa,
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
            market_value_zar: value.market_value_zar,
            base_cost_zar: value.base_cost_zar,
            is_liquid: value.is_liquid,
            situs_in_south_africa: value.situs_in_south_africa,
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
            debts_and_loans_zar: value.debts_and_loans_zar,
            funeral_costs_zar: value.funeral_costs_zar,
            administration_costs_zar: value.administration_costs_zar,
            masters_office_fees_zar: value.masters_office_fees_zar,
            conveyancing_costs_zar: value.conveyancing_costs_zar,
            other_settlement_costs_zar: value.other_settlement_costs_zar,
            final_income_tax_due_zar: value.final_income_tax_due_zar,
            ongoing_estate_income_tax_provision_zar: value.ongoing_estate_income_tax_provision_zar,
            additional_allowable_estate_duty_deductions_zar: value
                .additional_allowable_estate_duty_deductions_zar,
            ported_section_4a_abatement_zar: value.ported_section_4a_abatement_zar,
            primary_residence_cgt_exclusion_cap_zar: value.primary_residence_cgt_exclusion_cap_zar,
            executor_fee_rate: value.executor_fee_rate,
            vat_rate: value.vat_rate,
            explicit_executor_fee_zar: value.explicit_executor_fee_zar,
            external_liquidity_proceeds_zar: value.external_liquidity_proceeds_zar,
            cash_reserve_zar: value.cash_reserve_zar,
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
            assets: value.assets.into_iter().map(ApiEstateAssetInput::from).collect(),
            debts_and_loans_zar: value.debts_and_loans_zar,
            funeral_costs_zar: value.funeral_costs_zar,
            administration_costs_zar: value.administration_costs_zar,
            masters_office_fees_zar: value.masters_office_fees_zar,
            conveyancing_costs_zar: value.conveyancing_costs_zar,
            other_settlement_costs_zar: value.other_settlement_costs_zar,
            final_income_tax_due_zar: value.final_income_tax_due_zar,
            ongoing_estate_income_tax_provision_zar: value.ongoing_estate_income_tax_provision_zar,
            additional_allowable_estate_duty_deductions_zar: value
                .additional_allowable_estate_duty_deductions_zar,
            ported_section_4a_abatement_zar: value.ported_section_4a_abatement_zar,
            primary_residence_cgt_exclusion_cap_zar: value.primary_residence_cgt_exclusion_cap_zar,
            executor_fee_rate: value.executor_fee_rate,
            vat_rate: value.vat_rate,
            explicit_executor_fee_zar: value.explicit_executor_fee_zar,
            external_liquidity_proceeds_zar: value.external_liquidity_proceeds_zar,
            cash_reserve_zar: value.cash_reserve_zar,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiCapitalGainsTaxBreakdown {
    pub gross_capital_gain_zar: f64,
    pub primary_residence_exclusion_used_zar: f64,
    pub annual_exclusion_used_zar: f64,
    pub inclusion_rate: f64,
    pub taxable_capital_gain_in_income_zar: f64,
    pub tax_payable_zar: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiEstateDutyBreakdown {
    pub gross_estate_for_estate_duty_zar: f64,
    pub executor_fee_zar: f64,
    pub section_4q_spousal_deduction_zar: f64,
    pub pbo_deduction_zar: f64,
    pub total_allowable_deductions_zar: f64,
    pub section_4a_abatement_used_zar: f64,
    pub dutiable_estate_after_abatement_zar: f64,
    pub tax_payable_zar: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiCombinedTaxLiability {
    pub estate_duty_zar: f64,
    pub cgt_on_death_zar: f64,
    pub final_income_tax_zar: f64,
    pub ongoing_estate_income_tax_provision_zar: f64,
    pub total_tax_liability_zar: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiLiquidityGapOutput {
    pub liquid_assets_in_estate_zar: f64,
    pub external_liquidity_proceeds_zar: f64,
    pub cash_reserve_zar: f64,
    pub total_available_liquidity_zar: f64,
    pub executor_fee_zar: f64,
    pub immediate_cash_requirements_zar: f64,
    pub liquidity_gap_zar: f64,
    pub liquidity_surplus_zar: f64,
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
            gross_capital_gain_zar: value.gross_capital_gain_zar,
            primary_residence_exclusion_used_zar: value.primary_residence_exclusion_used_zar,
            annual_exclusion_used_zar: value.annual_exclusion_used_zar,
            inclusion_rate: value.inclusion_rate,
            taxable_capital_gain_in_income_zar: value.taxable_capital_gain_in_income_zar,
            tax_payable_zar: value.tax_payable_zar,
        }
    }
}

impl From<EstateDutyBreakdown> for ApiEstateDutyBreakdown {
    fn from(value: EstateDutyBreakdown) -> Self {
        ApiEstateDutyBreakdown {
            gross_estate_for_estate_duty_zar: value.gross_estate_for_estate_duty_zar,
            executor_fee_zar: value.executor_fee_zar,
            section_4q_spousal_deduction_zar: value.section_4q_spousal_deduction_zar,
            pbo_deduction_zar: value.pbo_deduction_zar,
            total_allowable_deductions_zar: value.total_allowable_deductions_zar,
            section_4a_abatement_used_zar: value.section_4a_abatement_used_zar,
            dutiable_estate_after_abatement_zar: value.dutiable_estate_after_abatement_zar,
            tax_payable_zar: value.tax_payable_zar,
        }
    }
}

impl From<CombinedTaxLiability> for ApiCombinedTaxLiability {
    fn from(value: CombinedTaxLiability) -> Self {
        ApiCombinedTaxLiability {
            estate_duty_zar: value.estate_duty_zar,
            cgt_on_death_zar: value.cgt_on_death_zar,
            final_income_tax_zar: value.final_income_tax_zar,
            ongoing_estate_income_tax_provision_zar: value.ongoing_estate_income_tax_provision_zar,
            total_tax_liability_zar: value.total_tax_liability_zar,
        }
    }
}

impl From<LiquidityGapOutput> for ApiLiquidityGapOutput {
    fn from(value: LiquidityGapOutput) -> Self {
        ApiLiquidityGapOutput {
            liquid_assets_in_estate_zar: value.liquid_assets_in_estate_zar,
            external_liquidity_proceeds_zar: value.external_liquidity_proceeds_zar,
            cash_reserve_zar: value.cash_reserve_zar,
            total_available_liquidity_zar: value.total_available_liquidity_zar,
            executor_fee_zar: value.executor_fee_zar,
            immediate_cash_requirements_zar: value.immediate_cash_requirements_zar,
            liquidity_gap_zar: value.liquidity_gap_zar,
            liquidity_surplus_zar: value.liquidity_surplus_zar,
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
