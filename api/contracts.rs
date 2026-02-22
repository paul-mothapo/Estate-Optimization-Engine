use crate::core::domain::models::{
    CapitalGainsTaxBreakdown, CombinedTaxLiability, EstateAsset, EstateDutyBreakdown,
    EstateScenarioInput, LiquidityGapOutput, ResidencyStatus, ScenarioResult,
};
use crate::core::engine::optimizer::OptimizedScenario;
use crate::core::engine::scoring::{LiquidityRiskBand, ScenarioScore};
use crate::core::rules::tax_rules::{
    CapitalGainsAtDeathRule, DonationsTaxRule, EstateDutyRule, Jurisdiction, JurisdictionTaxRuleSet,
    TaxPayerClass, TaxRuleRegistryEntry, TaxRuleVersion, VersionedJurisdictionTaxRuleSet,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JurisdictionTaxRuleRegistryResponse {
    pub jurisdiction: Jurisdiction,
    pub versions: Vec<TaxRuleVersion>,
    pub supported_tax_year_from: u16,
    pub supported_tax_year_to: Option<u16>,
    pub latest_version_id: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiJurisdiction {
    SouthAfrica,
}

impl From<Jurisdiction> for ApiJurisdiction {
    fn from(value: Jurisdiction) -> Self {
        match value {
            Jurisdiction::SouthAfrica => ApiJurisdiction::SouthAfrica,
        }
    }
}

impl From<ApiJurisdiction> for Jurisdiction {
    fn from(value: ApiJurisdiction) -> Self {
        match value {
            ApiJurisdiction::SouthAfrica => Jurisdiction::SouthAfrica,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiTaxRuleVersion {
    pub version_id: String,
    pub tax_year_from: u16,
    pub tax_year_to: Option<u16>,
    pub effective_from: String,
    pub effective_to: Option<String>,
    pub source_last_verified_on: String,
}

impl From<TaxRuleVersion> for ApiTaxRuleVersion {
    fn from(value: TaxRuleVersion) -> Self {
        ApiTaxRuleVersion {
            version_id: value.version_id.to_string(),
            tax_year_from: value.tax_year_from,
            tax_year_to: value.tax_year_to,
            effective_from: value.effective_from.to_string(),
            effective_to: value.effective_to.map(str::to_string),
            source_last_verified_on: value.source_last_verified_on.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiTaxRuleRegistryEntry {
    pub jurisdiction: ApiJurisdiction,
    pub version: ApiTaxRuleVersion,
}

impl From<TaxRuleRegistryEntry> for ApiTaxRuleRegistryEntry {
    fn from(value: TaxRuleRegistryEntry) -> Self {
        ApiTaxRuleRegistryEntry {
            jurisdiction: value.jurisdiction.into(),
            version: value.version.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiJurisdictionTaxRuleRegistryResponse {
    pub jurisdiction: ApiJurisdiction,
    pub versions: Vec<ApiTaxRuleVersion>,
    pub supported_tax_year_from: u16,
    pub supported_tax_year_to: Option<u16>,
    pub latest_version_id: String,
}

impl From<JurisdictionTaxRuleRegistryResponse> for ApiJurisdictionTaxRuleRegistryResponse {
    fn from(value: JurisdictionTaxRuleRegistryResponse) -> Self {
        ApiJurisdictionTaxRuleRegistryResponse {
            jurisdiction: value.jurisdiction.into(),
            versions: value.versions.into_iter().map(ApiTaxRuleVersion::from).collect(),
            supported_tax_year_from: value.supported_tax_year_from,
            supported_tax_year_to: value.supported_tax_year_to,
            latest_version_id: value.latest_version_id.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiEstateDutyRule {
    pub section_4a_abatement_zar: f64,
    pub primary_rate: f64,
    pub primary_rate_cap_zar: f64,
    pub secondary_rate: f64,
    pub spouse_deduction_unlimited: bool,
    pub effective_from: String,
    pub source: String,
    pub source_url: String,
}

impl From<EstateDutyRule> for ApiEstateDutyRule {
    fn from(value: EstateDutyRule) -> Self {
        ApiEstateDutyRule {
            section_4a_abatement_zar: value.section_4a_abatement_zar,
            primary_rate: value.primary_rate,
            primary_rate_cap_zar: value.primary_rate_cap_zar,
            secondary_rate: value.secondary_rate,
            spouse_deduction_unlimited: value.spouse_deduction_unlimited,
            effective_from: value.effective_from.to_string(),
            source: value.source.to_string(),
            source_url: value.source_url.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiDonationsTaxRule {
    pub annual_exemption_natural_person_zar: f64,
    pub annual_exemption_non_natural_casual_gifts_zar: f64,
    pub primary_rate: f64,
    pub primary_rate_cap_cumulative_zar: f64,
    pub secondary_rate: f64,
    pub effective_from: String,
    pub source: String,
    pub source_url: String,
}

impl From<DonationsTaxRule> for ApiDonationsTaxRule {
    fn from(value: DonationsTaxRule) -> Self {
        ApiDonationsTaxRule {
            annual_exemption_natural_person_zar: value.annual_exemption_natural_person_zar,
            annual_exemption_non_natural_casual_gifts_zar: value
                .annual_exemption_non_natural_casual_gifts_zar,
            primary_rate: value.primary_rate,
            primary_rate_cap_cumulative_zar: value.primary_rate_cap_cumulative_zar,
            secondary_rate: value.secondary_rate,
            effective_from: value.effective_from.to_string(),
            source: value.source.to_string(),
            source_url: value.source_url.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiCapitalGainsAtDeathRule {
    pub annual_exclusion_in_year_of_death_zar: f64,
    pub inclusion_rate_natural_person: f64,
    pub inclusion_rate_company: f64,
    pub inclusion_rate_trust: f64,
    pub base_cost_step_up_to_market_value_on_death: bool,
    pub effective_from: String,
    pub source: String,
    pub source_url: String,
}

impl From<CapitalGainsAtDeathRule> for ApiCapitalGainsAtDeathRule {
    fn from(value: CapitalGainsAtDeathRule) -> Self {
        ApiCapitalGainsAtDeathRule {
            annual_exclusion_in_year_of_death_zar: value.annual_exclusion_in_year_of_death_zar,
            inclusion_rate_natural_person: value.inclusion_rate_natural_person,
            inclusion_rate_company: value.inclusion_rate_company,
            inclusion_rate_trust: value.inclusion_rate_trust,
            base_cost_step_up_to_market_value_on_death: value.base_cost_step_up_to_market_value_on_death,
            effective_from: value.effective_from.to_string(),
            source: value.source.to_string(),
            source_url: value.source_url.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiJurisdictionTaxRuleSet {
    pub estate_duty: ApiEstateDutyRule,
    pub donations_tax: ApiDonationsTaxRule,
    pub cgt_on_death: ApiCapitalGainsAtDeathRule,
}

impl From<JurisdictionTaxRuleSet> for ApiJurisdictionTaxRuleSet {
    fn from(value: JurisdictionTaxRuleSet) -> Self {
        ApiJurisdictionTaxRuleSet {
            estate_duty: value.estate_duty.into(),
            donations_tax: value.donations_tax.into(),
            cgt_on_death: value.cgt_on_death.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiVersionedJurisdictionTaxRuleSet {
    pub version: ApiTaxRuleVersion,
    pub rules: ApiJurisdictionTaxRuleSet,
}

impl From<VersionedJurisdictionTaxRuleSet> for ApiVersionedJurisdictionTaxRuleSet {
    fn from(value: VersionedJurisdictionTaxRuleSet) -> Self {
        ApiVersionedJurisdictionTaxRuleSet {
            version: value.version.into(),
            rules: value.rules.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiErrorCode {
    Validation,
    RuleSelection,
    Computation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiValidationIssue {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub code: ApiErrorCode,
    pub message: String,
    pub validation_issues: Vec<ApiValidationIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiHealthResponse {
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiCapitalGainsTaxBreakdown {
    pub gross_capital_gain_zar: f64,
    pub primary_residence_exclusion_used_zar: f64,
    pub annual_exclusion_used_zar: f64,
    pub inclusion_rate: f64,
    pub taxable_capital_gain_in_income_zar: f64,
    pub tax_payable_zar: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiCombinedTaxLiability {
    pub estate_duty_zar: f64,
    pub cgt_on_death_zar: f64,
    pub final_income_tax_zar: f64,
    pub ongoing_estate_income_tax_provision_zar: f64,
    pub total_tax_liability_zar: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
