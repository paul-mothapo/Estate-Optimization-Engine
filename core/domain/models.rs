use crate::core::rules::tax_rules::{Jurisdiction, TaxPayerClass};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResidencyStatus {
    Resident,
    NonResident,
}

#[derive(Debug, Clone)]
pub struct EstateAsset {
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

impl EstateAsset {
    pub fn raw_capital_gain_zar(&self) -> f64 {
        (self.market_value_zar - self.base_cost_zar).max(0.0)
    }
}

#[derive(Debug, Clone)]
pub struct EstateScenarioInput {
    pub jurisdiction: Jurisdiction,
    pub tax_year: u16,
    pub taxpayer_class: TaxPayerClass,
    pub residency_status: ResidencyStatus,
    pub marginal_income_tax_rate: f64,
    pub assets: Vec<EstateAsset>,
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

impl Default for EstateScenarioInput {
    fn default() -> Self {
        Self {
            jurisdiction: Jurisdiction::SouthAfrica,
            tax_year: 2026,
            taxpayer_class: TaxPayerClass::NaturalPerson,
            residency_status: ResidencyStatus::Resident,
            marginal_income_tax_rate: 0.45,
            assets: Vec::new(),
            debts_and_loans_zar: 0.0,
            funeral_costs_zar: 0.0,
            administration_costs_zar: 0.0,
            masters_office_fees_zar: 0.0,
            conveyancing_costs_zar: 0.0,
            other_settlement_costs_zar: 0.0,
            final_income_tax_due_zar: 0.0,
            ongoing_estate_income_tax_provision_zar: 0.0,
            additional_allowable_estate_duty_deductions_zar: 0.0,
            ported_section_4a_abatement_zar: 0.0,
            primary_residence_cgt_exclusion_cap_zar: 2_000_000.0,
            executor_fee_rate: 0.035,
            vat_rate: 0.15,
            explicit_executor_fee_zar: None,
            external_liquidity_proceeds_zar: 0.0,
            cash_reserve_zar: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CapitalGainsTaxBreakdown {
    pub gross_capital_gain_zar: f64,
    pub primary_residence_exclusion_used_zar: f64,
    pub annual_exclusion_used_zar: f64,
    pub inclusion_rate: f64,
    pub taxable_capital_gain_in_income_zar: f64,
    pub tax_payable_zar: f64,
}

#[derive(Debug, Clone)]
pub struct EstateDutyBreakdown {
    pub gross_estate_for_estate_duty_zar: f64,
    pub executor_fee_zar: f64,
    pub section_4q_spousal_deduction_zar: f64,
    pub pbo_deduction_zar: f64,
    pub total_allowable_deductions_zar: f64,
    pub section_4a_abatement_used_zar: f64,
    pub dutiable_estate_after_abatement_zar: f64,
    pub tax_payable_zar: f64,
}

#[derive(Debug, Clone)]
pub struct CombinedTaxLiability {
    pub estate_duty_zar: f64,
    pub cgt_on_death_zar: f64,
    pub final_income_tax_zar: f64,
    pub ongoing_estate_income_tax_provision_zar: f64,
    pub total_tax_liability_zar: f64,
}

#[derive(Debug, Clone)]
pub struct LiquidityGapOutput {
    pub liquid_assets_in_estate_zar: f64,
    pub external_liquidity_proceeds_zar: f64,
    pub cash_reserve_zar: f64,
    pub total_available_liquidity_zar: f64,
    pub executor_fee_zar: f64,
    pub immediate_cash_requirements_zar: f64,
    pub liquidity_gap_zar: f64,
    pub liquidity_surplus_zar: f64,
}

#[derive(Debug, Clone)]
pub struct ScenarioResult {
    pub cgt: CapitalGainsTaxBreakdown,
    pub estate_duty: EstateDutyBreakdown,
    pub combined_tax: CombinedTaxLiability,
    pub liquidity: LiquidityGapOutput,
}
