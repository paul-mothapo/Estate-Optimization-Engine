use crate::core::models::{
    CapitalGainsTaxBreakdown, CombinedTaxLiability, EstateDutyBreakdown, EstateScenarioInput,
    LiquidityGapOutput, ResidencyStatus, ScenarioResult,
};
use crate::core::tax_rules::{baseline_tax_rules_for, JurisdictionTaxRuleSet, TaxPayerClass};

pub trait ScenarioCalculator {
    fn calculate(&self, input: &EstateScenarioInput) -> ScenarioResult;
}

#[derive(Debug, Clone)]
pub struct SouthAfricaScenarioCalculator {
    rules: JurisdictionTaxRuleSet,
}

impl SouthAfricaScenarioCalculator {
    pub fn new(input: &EstateScenarioInput) -> Self {
        Self {
            rules: baseline_tax_rules_for(input.jurisdiction),
        }
    }

    fn clamp_rate(rate: f64) -> f64 {
        rate.clamp(0.0, 1.0)
    }

    fn asset_in_estate_duty_scope(
        input: &EstateScenarioInput,
        included_in_estate_duty: bool,
        situs_in_south_africa: bool,
    ) -> bool {
        if !included_in_estate_duty {
            return false;
        }

        match input.residency_status {
            ResidencyStatus::Resident => true,
            ResidencyStatus::NonResident => situs_in_south_africa,
        }
    }

    fn calculate_cgt(&self, input: &EstateScenarioInput) -> CapitalGainsTaxBreakdown {
        let marginal_income_tax_rate = Self::clamp_rate(input.marginal_income_tax_rate);
        let mut gross_capital_gain_zar = 0.0;
        let mut primary_residence_exclusion_remaining_zar =
            input.primary_residence_cgt_exclusion_cap_zar.max(0.0);
        let mut primary_residence_exclusion_used_zar = 0.0;

        for asset in &input.assets {
            if !asset.included_in_cgt_deemed_disposal {
                continue;
            }

            let mut gain = asset.raw_capital_gain_zar();
            if asset.qualifies_primary_residence_exclusion
                && primary_residence_exclusion_remaining_zar > 0.0
            {
                let exclusion = gain.min(primary_residence_exclusion_remaining_zar);
                primary_residence_exclusion_used_zar += exclusion;
                primary_residence_exclusion_remaining_zar -= exclusion;
                gain -= exclusion;
            }

            gross_capital_gain_zar += gain;
        }

        let annual_exclusion_used_zar = match input.taxpayer_class {
            TaxPayerClass::NaturalPerson | TaxPayerClass::SpecialTrust => gross_capital_gain_zar
                .min(self.rules.cgt_on_death.annual_exclusion_in_year_of_death_zar),
            TaxPayerClass::Company | TaxPayerClass::Trust => 0.0,
        };

        let inclusion_rate = self.rules.cgt_on_death.inclusion_rate_for(input.taxpayer_class);
        let taxable_capital_gain_in_income_zar =
            (gross_capital_gain_zar - annual_exclusion_used_zar).max(0.0) * inclusion_rate;
        let tax_payable_zar = taxable_capital_gain_in_income_zar * marginal_income_tax_rate;

        CapitalGainsTaxBreakdown {
            gross_capital_gain_zar,
            primary_residence_exclusion_used_zar,
            annual_exclusion_used_zar,
            inclusion_rate,
            taxable_capital_gain_in_income_zar,
            tax_payable_zar,
        }
    }

    fn calculate_estate_duty(
        &self,
        input: &EstateScenarioInput,
        cgt_tax_payable_zar: f64,
    ) -> EstateDutyBreakdown {
        let gross_estate_for_estate_duty_zar = input
            .assets
            .iter()
            .filter(|asset| {
                Self::asset_in_estate_duty_scope(
                    input,
                    asset.included_in_estate_duty,
                    asset.situs_in_south_africa,
                )
            })
            .map(|asset| asset.market_value_zar.max(0.0))
            .sum::<f64>();

        let gross_estate_for_executor_fee_zar = input
            .assets
            .iter()
            .map(|asset| asset.market_value_zar.max(0.0))
            .sum::<f64>();

        let executor_fee_zar = input.explicit_executor_fee_zar.unwrap_or_else(|| {
            let fee_rate = Self::clamp_rate(input.executor_fee_rate);
            let vat_rate = Self::clamp_rate(input.vat_rate);
            gross_estate_for_executor_fee_zar * fee_rate * (1.0 + vat_rate)
        });

        let section_4q_spousal_deduction_zar = if self.rules.estate_duty.spouse_deduction_unlimited {
            input
                .assets
                .iter()
                .filter(|asset| {
                    asset.bequeathed_to_surviving_spouse
                        && Self::asset_in_estate_duty_scope(
                            input,
                            asset.included_in_estate_duty,
                            asset.situs_in_south_africa,
                        )
                })
                .map(|asset| asset.market_value_zar.max(0.0))
                .sum::<f64>()
        } else {
            0.0
        };

        let pbo_deduction_zar = input
            .assets
            .iter()
            .filter(|asset| {
                asset.bequeathed_to_pbo
                    && Self::asset_in_estate_duty_scope(
                        input,
                        asset.included_in_estate_duty,
                        asset.situs_in_south_africa,
                    )
            })
            .map(|asset| asset.market_value_zar.max(0.0))
            .sum::<f64>();

        let total_allowable_deductions_zar = input.debts_and_loans_zar.max(0.0)
            + input.funeral_costs_zar.max(0.0)
            + input.administration_costs_zar.max(0.0)
            + input.masters_office_fees_zar.max(0.0)
            + input.conveyancing_costs_zar.max(0.0)
            + input.other_settlement_costs_zar.max(0.0)
            + input.final_income_tax_due_zar.max(0.0)
            + input.ongoing_estate_income_tax_provision_zar.max(0.0)
            + cgt_tax_payable_zar
            + executor_fee_zar.max(0.0)
            + section_4q_spousal_deduction_zar
            + pbo_deduction_zar
            + input.additional_allowable_estate_duty_deductions_zar.max(0.0);

        let net_estate_before_abatement_zar =
            (gross_estate_for_estate_duty_zar - total_allowable_deductions_zar).max(0.0);
        let section_4a_abatement_used_zar = self.rules.estate_duty.section_4a_abatement_zar
            + input.ported_section_4a_abatement_zar.max(0.0);
        let dutiable_estate_after_abatement_zar =
            (net_estate_before_abatement_zar - section_4a_abatement_used_zar).max(0.0);

        let primary_band =
            dutiable_estate_after_abatement_zar.min(self.rules.estate_duty.primary_rate_cap_zar);
        let secondary_band =
            (dutiable_estate_after_abatement_zar - self.rules.estate_duty.primary_rate_cap_zar)
                .max(0.0);
        let tax_payable_zar = primary_band * self.rules.estate_duty.primary_rate
            + secondary_band * self.rules.estate_duty.secondary_rate;

        EstateDutyBreakdown {
            gross_estate_for_estate_duty_zar,
            executor_fee_zar,
            section_4q_spousal_deduction_zar,
            pbo_deduction_zar,
            total_allowable_deductions_zar,
            section_4a_abatement_used_zar,
            dutiable_estate_after_abatement_zar,
            tax_payable_zar,
        }
    }

    fn calculate_combined_tax(
        &self,
        input: &EstateScenarioInput,
        cgt_tax_payable_zar: f64,
        estate_duty_tax_payable_zar: f64,
    ) -> CombinedTaxLiability {
        let final_income_tax_zar = input.final_income_tax_due_zar.max(0.0);
        let ongoing_income_tax_zar = input.ongoing_estate_income_tax_provision_zar.max(0.0);
        let total_tax_liability_zar = estate_duty_tax_payable_zar
            + cgt_tax_payable_zar
            + final_income_tax_zar
            + ongoing_income_tax_zar;

        CombinedTaxLiability {
            estate_duty_zar: estate_duty_tax_payable_zar,
            cgt_on_death_zar: cgt_tax_payable_zar,
            final_income_tax_zar,
            ongoing_estate_income_tax_provision_zar: ongoing_income_tax_zar,
            total_tax_liability_zar,
        }
    }

    fn calculate_liquidity(
        &self,
        input: &EstateScenarioInput,
        combined_tax: &CombinedTaxLiability,
        executor_fee_zar: f64,
    ) -> LiquidityGapOutput {
        let liquid_assets_in_estate_zar = input
            .assets
            .iter()
            .filter(|asset| asset.is_liquid)
            .map(|asset| asset.market_value_zar.max(0.0))
            .sum::<f64>();

        let immediate_cash_requirements_zar = combined_tax.total_tax_liability_zar
            + input.debts_and_loans_zar.max(0.0)
            + input.funeral_costs_zar.max(0.0)
            + input.administration_costs_zar.max(0.0)
            + executor_fee_zar.max(0.0)
            + input.masters_office_fees_zar.max(0.0)
            + input.conveyancing_costs_zar.max(0.0)
            + input.other_settlement_costs_zar.max(0.0);

        let external_liquidity_proceeds_zar = input.external_liquidity_proceeds_zar.max(0.0);
        let cash_reserve_zar = input.cash_reserve_zar.max(0.0);
        let total_available_liquidity_zar =
            liquid_assets_in_estate_zar + external_liquidity_proceeds_zar + cash_reserve_zar;
        let liquidity_gap_zar =
            (immediate_cash_requirements_zar - total_available_liquidity_zar).max(0.0);
        let liquidity_surplus_zar =
            (total_available_liquidity_zar - immediate_cash_requirements_zar).max(0.0);

        LiquidityGapOutput {
            liquid_assets_in_estate_zar,
            external_liquidity_proceeds_zar,
            cash_reserve_zar,
            total_available_liquidity_zar,
            executor_fee_zar: executor_fee_zar.max(0.0),
            immediate_cash_requirements_zar,
            liquidity_gap_zar,
            liquidity_surplus_zar,
        }
    }
}

impl ScenarioCalculator for SouthAfricaScenarioCalculator {
    fn calculate(&self, input: &EstateScenarioInput) -> ScenarioResult {
        let cgt = self.calculate_cgt(input);
        let estate_duty = self.calculate_estate_duty(input, cgt.tax_payable_zar);
        let combined_tax =
            self.calculate_combined_tax(input, cgt.tax_payable_zar, estate_duty.tax_payable_zar);
        let liquidity = self.calculate_liquidity(input, &combined_tax, estate_duty.executor_fee_zar);

        ScenarioResult {
            cgt,
            estate_duty,
            combined_tax,
            liquidity,
        }
    }
}

pub fn calculate_combined_tax_and_liquidity(input: &EstateScenarioInput) -> ScenarioResult {
    let calculator = SouthAfricaScenarioCalculator::new(input);
    calculator.calculate(input)
}
