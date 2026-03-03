use crate::core::domain::models::{
    CapitalGainsTaxBreakdown, CombinedTaxLiability, EstateDutyBreakdown, EstateScenarioInput,
    LiquidityGapOutput, ResidencyStatus, ScenarioResult,
};
use crate::core::rules::tax_rules::{
    tax_rules_for, JurisdictionTaxRuleSet, TaxPayerClass, TaxRuleSelectionError,
};

pub trait ScenarioCalculator {
    fn calculate(&self, input: &EstateScenarioInput) -> ScenarioResult;
}

#[derive(Debug, Clone)]
pub struct JurisdictionScenarioCalculator {
    rules: JurisdictionTaxRuleSet,
}

impl JurisdictionScenarioCalculator {
    pub fn new(input: &EstateScenarioInput) -> Result<Self, TaxRuleSelectionError> {
        let selected = tax_rules_for(input.jurisdiction, input.tax_year)?;
        Ok(Self {
            rules: selected.rules,
        })
    }

    fn clamp_rate(rate: f64) -> f64 {
        rate.clamp(0.0, 1.0)
    }

    fn asset_in_estate_duty_scope(
        input: &EstateScenarioInput,
        included_in_estate_duty: bool,
        situs_in_jurisdiction: bool,
    ) -> bool {
        if !included_in_estate_duty {
            return false;
        }

        match input.residency_status {
            ResidencyStatus::Resident => true,
            ResidencyStatus::NonResident => situs_in_jurisdiction,
        }
    }

    fn calculate_cgt(&self, input: &EstateScenarioInput) -> CapitalGainsTaxBreakdown {
        let marginal_income_tax_rate = Self::clamp_rate(input.marginal_income_tax_rate);
        let mut gross_capital_gain_amount = 0.0;
        let mut primary_residence_exclusion_remaining_amount =
            input.primary_residence_cgt_exclusion_cap_amount.max(0.0);
        let mut primary_residence_exclusion_used_amount = 0.0;

        for asset in &input.assets {
            if !asset.included_in_cgt_deemed_disposal {
                continue;
            }

            let mut gain = asset.raw_capital_gain_amount();
            if asset.qualifies_primary_residence_exclusion
                && primary_residence_exclusion_remaining_amount > 0.0
            {
                let exclusion = gain.min(primary_residence_exclusion_remaining_amount);
                primary_residence_exclusion_used_amount += exclusion;
                primary_residence_exclusion_remaining_amount -= exclusion;
                gain -= exclusion;
            }

            gross_capital_gain_amount += gain;
        }

        let annual_exclusion_used_amount = match input.taxpayer_class {
            TaxPayerClass::NaturalPerson | TaxPayerClass::SpecialTrust => gross_capital_gain_amount
                .min(
                    self.rules
                        .cgt_on_death
                        .annual_exclusion_in_year_of_death_amount,
                ),
            TaxPayerClass::Company | TaxPayerClass::Trust => 0.0,
        };

        let inclusion_rate = self
            .rules
            .cgt_on_death
            .inclusion_rate_for(input.taxpayer_class);
        let taxable_capital_gain_in_income_amount =
            (gross_capital_gain_amount - annual_exclusion_used_amount).max(0.0) * inclusion_rate;
        let tax_payable_amount = taxable_capital_gain_in_income_amount * marginal_income_tax_rate;

        CapitalGainsTaxBreakdown {
            gross_capital_gain_amount,
            primary_residence_exclusion_used_amount,
            annual_exclusion_used_amount,
            inclusion_rate,
            taxable_capital_gain_in_income_amount,
            tax_payable_amount,
        }
    }

    fn calculate_estate_duty(
        &self,
        input: &EstateScenarioInput,
        cgt_tax_payable_amount: f64,
    ) -> EstateDutyBreakdown {
        let gross_estate_for_transfer_tax_amount = input
            .assets
            .iter()
            .filter(|asset| {
                Self::asset_in_estate_duty_scope(
                    input,
                    asset.included_in_estate_duty,
                    asset.situs_in_jurisdiction,
                )
            })
            .map(|asset| asset.market_value_amount.max(0.0))
            .sum::<f64>();

        let gross_estate_for_executor_fee_amount = input
            .assets
            .iter()
            .map(|asset| asset.market_value_amount.max(0.0))
            .sum::<f64>();

        let executor_fee_amount = input.explicit_executor_fee_amount.unwrap_or_else(|| {
            let fee_rate = Self::clamp_rate(input.executor_fee_rate);
            let vat_rate = Self::clamp_rate(input.vat_rate);
            gross_estate_for_executor_fee_amount * fee_rate * (1.0 + vat_rate)
        });

        let spousal_deduction_amount = if self.rules.estate_duty.spouse_deduction_unlimited {
            input
                .assets
                .iter()
                .filter(|asset| {
                    asset.bequeathed_to_surviving_spouse
                        && Self::asset_in_estate_duty_scope(
                            input,
                            asset.included_in_estate_duty,
                            asset.situs_in_jurisdiction,
                        )
                })
                .map(|asset| asset.market_value_amount.max(0.0))
                .sum::<f64>()
        } else {
            0.0
        };

        let pbo_deduction_amount = input
            .assets
            .iter()
            .filter(|asset| {
                asset.bequeathed_to_pbo
                    && Self::asset_in_estate_duty_scope(
                        input,
                        asset.included_in_estate_duty,
                        asset.situs_in_jurisdiction,
                    )
            })
            .map(|asset| asset.market_value_amount.max(0.0))
            .sum::<f64>();

        let total_allowable_deductions_amount = input.debts_and_loans_amount.max(0.0)
            + input.funeral_costs_amount.max(0.0)
            + input.administration_costs_amount.max(0.0)
            + input.masters_office_fees_amount.max(0.0)
            + input.conveyancing_costs_amount.max(0.0)
            + input.other_settlement_costs_amount.max(0.0)
            + input.final_income_tax_due_amount.max(0.0)
            + input.ongoing_estate_income_tax_provision_amount.max(0.0)
            + cgt_tax_payable_amount
            + executor_fee_amount.max(0.0)
            + spousal_deduction_amount
            + pbo_deduction_amount
            + input
                .additional_allowable_estate_transfer_tax_deductions_amount
                .max(0.0);

        let net_estate_before_exemption_amount =
            (gross_estate_for_transfer_tax_amount - total_allowable_deductions_amount).max(0.0);
        let exemption_used_amount = self.rules.estate_duty.exemption_amount
            + input.ported_estate_tax_exemption_amount.max(0.0);
        let dutiable_estate_after_exemption_amount =
            (net_estate_before_exemption_amount - exemption_used_amount).max(0.0);

        let primary_band = dutiable_estate_after_exemption_amount
            .min(self.rules.estate_duty.primary_rate_cap_amount);
        let secondary_band = (dutiable_estate_after_exemption_amount
            - self.rules.estate_duty.primary_rate_cap_amount)
            .max(0.0);
        let tax_payable_amount = primary_band * self.rules.estate_duty.primary_rate
            + secondary_band * self.rules.estate_duty.secondary_rate;

        EstateDutyBreakdown {
            gross_estate_for_transfer_tax_amount,
            executor_fee_amount,
            spousal_deduction_amount,
            pbo_deduction_amount,
            total_allowable_deductions_amount,
            exemption_used_amount,
            dutiable_estate_after_exemption_amount,
            tax_payable_amount,
        }
    }

    fn calculate_combined_tax(
        &self,
        input: &EstateScenarioInput,
        cgt_tax_payable_amount: f64,
        estate_duty_tax_payable_amount: f64,
    ) -> CombinedTaxLiability {
        let final_income_tax_amount = input.final_income_tax_due_amount.max(0.0);
        let ongoing_income_tax_amount = input.ongoing_estate_income_tax_provision_amount.max(0.0);
        let total_tax_liability_amount = estate_duty_tax_payable_amount
            + cgt_tax_payable_amount
            + final_income_tax_amount
            + ongoing_income_tax_amount;

        CombinedTaxLiability {
            estate_transfer_tax_amount: estate_duty_tax_payable_amount,
            cgt_on_death_amount: cgt_tax_payable_amount,
            final_income_tax_amount,
            ongoing_estate_income_tax_provision_amount: ongoing_income_tax_amount,
            total_tax_liability_amount,
        }
    }

    fn calculate_liquidity(
        &self,
        input: &EstateScenarioInput,
        combined_tax: &CombinedTaxLiability,
        executor_fee_amount: f64,
    ) -> LiquidityGapOutput {
        let liquid_assets_in_estate_amount = input
            .assets
            .iter()
            .filter(|asset| asset.is_liquid)
            .map(|asset| asset.market_value_amount.max(0.0))
            .sum::<f64>();

        let immediate_cash_requirements_amount = combined_tax.total_tax_liability_amount
            + input.debts_and_loans_amount.max(0.0)
            + input.funeral_costs_amount.max(0.0)
            + input.administration_costs_amount.max(0.0)
            + executor_fee_amount.max(0.0)
            + input.masters_office_fees_amount.max(0.0)
            + input.conveyancing_costs_amount.max(0.0)
            + input.other_settlement_costs_amount.max(0.0);

        let external_liquidity_proceeds_amount = input.external_liquidity_proceeds_amount.max(0.0);
        let cash_reserve_amount = input.cash_reserve_amount.max(0.0);
        let total_available_liquidity_amount = liquid_assets_in_estate_amount
            + external_liquidity_proceeds_amount
            + cash_reserve_amount;
        let liquidity_gap_amount =
            (immediate_cash_requirements_amount - total_available_liquidity_amount).max(0.0);
        let liquidity_surplus_amount =
            (total_available_liquidity_amount - immediate_cash_requirements_amount).max(0.0);

        LiquidityGapOutput {
            liquid_assets_in_estate_amount,
            external_liquidity_proceeds_amount,
            cash_reserve_amount,
            total_available_liquidity_amount,
            executor_fee_amount: executor_fee_amount.max(0.0),
            immediate_cash_requirements_amount,
            liquidity_gap_amount,
            liquidity_surplus_amount,
        }
    }
}

impl ScenarioCalculator for JurisdictionScenarioCalculator {
    fn calculate(&self, input: &EstateScenarioInput) -> ScenarioResult {
        let cgt = self.calculate_cgt(input);
        let estate_duty = self.calculate_estate_duty(input, cgt.tax_payable_amount);
        let combined_tax = self.calculate_combined_tax(
            input,
            cgt.tax_payable_amount,
            estate_duty.tax_payable_amount,
        );
        let liquidity =
            self.calculate_liquidity(input, &combined_tax, estate_duty.executor_fee_amount);

        ScenarioResult {
            cgt,
            estate_duty,
            combined_tax,
            liquidity,
        }
    }
}

pub fn calculate_combined_tax_and_liquidity(
    input: &EstateScenarioInput,
) -> Result<ScenarioResult, TaxRuleSelectionError> {
    let calculator = JurisdictionScenarioCalculator::new(input)?;
    Ok(calculator.calculate(input))
}
