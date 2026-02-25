use crate::core::domain::models::{EstateAsset, EstateScenarioInput, ResidencyStatus};
use crate::core::rules::tax_rules::{is_supported_tax_year, TaxPayerClass};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    pub field: String,
    pub message: String,
}

impl ValidationIssue {
    pub fn new(field: String, message: impl Into<String>) -> Self {
        Self {
            field,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputValidationError {
    pub issues: Vec<ValidationIssue>,
}

impl InputValidationError {
    pub fn new(issues: Vec<ValidationIssue>) -> Self {
        Self { issues }
    }

    pub fn is_empty(&self) -> bool {
        self.issues.is_empty()
    }
}

impl fmt::Display for InputValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.issues.is_empty() {
            return write!(f, "No validation issues");
        }

        writeln!(f, "Input validation failed:")?;
        for issue in &self.issues {
            writeln!(f, "- {}: {}", issue.field, issue.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for InputValidationError {}

fn check_non_negative_finite(issues: &mut Vec<ValidationIssue>, field: String, value: f64) {
    if !value.is_finite() {
        issues.push(ValidationIssue::new(field, "Value must be finite"));
        return;
    }
    if value < 0.0 {
        issues.push(ValidationIssue::new(field, "Value cannot be negative"));
    }
}

fn check_rate_inclusive(issues: &mut Vec<ValidationIssue>, field: String, value: f64) {
    if !value.is_finite() {
        issues.push(ValidationIssue::new(field, "Rate must be finite"));
        return;
    }
    if !(0.0..=1.0).contains(&value) {
        issues.push(ValidationIssue::new(
            field,
            "Rate must be between 0.0 and 1.0 (inclusive)",
        ));
    }
}

impl EstateAsset {
    fn validate_contract(
        &self,
        index: usize,
        taxpayer_class: TaxPayerClass,
        residency_status: ResidencyStatus,
        issues: &mut Vec<ValidationIssue>,
    ) {
        let prefix = format!("assets[{index}]");

        if self.name.trim().is_empty() {
            issues.push(ValidationIssue::new(
                format!("{prefix}.name"),
                "Asset name cannot be empty",
            ));
        }

        check_non_negative_finite(
            issues,
            format!("{prefix}.market_value_zar"),
            self.market_value_zar,
        );
        check_non_negative_finite(
            issues,
            format!("{prefix}.base_cost_zar"),
            self.base_cost_zar,
        );

        if self.bequeathed_to_surviving_spouse && self.bequeathed_to_pbo {
            issues.push(ValidationIssue::new(
                format!("{prefix}.bequests"),
                "Asset cannot be bequeathed to both spouse and PBO",
            ));
        }

        if !self.included_in_estate_duty
            && (self.bequeathed_to_surviving_spouse || self.bequeathed_to_pbo)
        {
            issues.push(ValidationIssue::new(
                format!("{prefix}.included_in_estate_duty"),
                "Spouse/PBO bequest flags require `included_in_estate_duty=true`",
            ));
        }

        if self.qualifies_primary_residence_exclusion && !self.included_in_cgt_deemed_disposal {
            issues.push(ValidationIssue::new(
                format!("{prefix}.included_in_cgt_deemed_disposal"),
                "Primary residence exclusion requires `included_in_cgt_deemed_disposal=true`",
            ));
        }

        if self.qualifies_primary_residence_exclusion
            && matches!(
                taxpayer_class,
                TaxPayerClass::Company | TaxPayerClass::Trust
            )
        {
            issues.push(ValidationIssue::new(
                format!("{prefix}.qualifies_primary_residence_exclusion"),
                "Primary residence exclusion is not supported for company/trust taxpayer class",
            ));
        }

        if matches!(residency_status, ResidencyStatus::NonResident)
            && self.included_in_estate_duty
            && !self.situs_in_south_africa
        {
            issues.push(ValidationIssue::new(
                format!("{prefix}.situs_in_south_africa"),
                "Non-resident estate duty scope requires SA situs for included assets",
            ));
        }
    }
}

impl EstateScenarioInput {
    pub fn validate(&self) -> Result<(), InputValidationError> {
        let mut issues = Vec::new();

        if !is_supported_tax_year(self.jurisdiction, self.tax_year) {
            issues.push(ValidationIssue::new(
                "tax_year".to_string(),
                format!(
                    "Tax year {} is not supported for {:?}",
                    self.tax_year, self.jurisdiction
                ),
            ));
        }

        if self.assets.is_empty() {
            issues.push(ValidationIssue::new(
                "assets".to_string(),
                "At least one asset is required",
            ));
        }

        let assets_with_positive_value = self
            .assets
            .iter()
            .any(|asset| asset.market_value_zar.is_finite() && asset.market_value_zar > 0.0);
        if !self.assets.is_empty() && !assets_with_positive_value {
            issues.push(ValidationIssue::new(
                "assets".to_string(),
                "At least one asset must have `market_value_zar > 0`",
            ));
        }

        check_rate_inclusive(
            &mut issues,
            "marginal_income_tax_rate".to_string(),
            self.marginal_income_tax_rate,
        );
        check_rate_inclusive(
            &mut issues,
            "executor_fee_rate".to_string(),
            self.executor_fee_rate,
        );
        check_rate_inclusive(&mut issues, "vat_rate".to_string(), self.vat_rate);

        check_non_negative_finite(
            &mut issues,
            "debts_and_loans_zar".to_string(),
            self.debts_and_loans_zar,
        );
        check_non_negative_finite(
            &mut issues,
            "funeral_costs_zar".to_string(),
            self.funeral_costs_zar,
        );
        check_non_negative_finite(
            &mut issues,
            "administration_costs_zar".to_string(),
            self.administration_costs_zar,
        );
        check_non_negative_finite(
            &mut issues,
            "masters_office_fees_zar".to_string(),
            self.masters_office_fees_zar,
        );
        check_non_negative_finite(
            &mut issues,
            "conveyancing_costs_zar".to_string(),
            self.conveyancing_costs_zar,
        );
        check_non_negative_finite(
            &mut issues,
            "other_settlement_costs_zar".to_string(),
            self.other_settlement_costs_zar,
        );
        check_non_negative_finite(
            &mut issues,
            "final_income_tax_due_zar".to_string(),
            self.final_income_tax_due_zar,
        );
        check_non_negative_finite(
            &mut issues,
            "ongoing_estate_income_tax_provision_zar".to_string(),
            self.ongoing_estate_income_tax_provision_zar,
        );
        check_non_negative_finite(
            &mut issues,
            "additional_allowable_estate_duty_deductions_zar".to_string(),
            self.additional_allowable_estate_duty_deductions_zar,
        );
        check_non_negative_finite(
            &mut issues,
            "ported_section_4a_abatement_zar".to_string(),
            self.ported_section_4a_abatement_zar,
        );
        check_non_negative_finite(
            &mut issues,
            "primary_residence_cgt_exclusion_cap_zar".to_string(),
            self.primary_residence_cgt_exclusion_cap_zar,
        );
        check_non_negative_finite(
            &mut issues,
            "external_liquidity_proceeds_zar".to_string(),
            self.external_liquidity_proceeds_zar,
        );
        check_non_negative_finite(
            &mut issues,
            "cash_reserve_zar".to_string(),
            self.cash_reserve_zar,
        );

        if let Some(explicit_executor_fee_zar) = self.explicit_executor_fee_zar {
            check_non_negative_finite(
                &mut issues,
                "explicit_executor_fee_zar".to_string(),
                explicit_executor_fee_zar,
            );
        }

        if matches!(
            self.taxpayer_class,
            TaxPayerClass::Company | TaxPayerClass::Trust
        ) && self.primary_residence_cgt_exclusion_cap_zar > 0.0
        {
            issues.push(ValidationIssue::new(
                "primary_residence_cgt_exclusion_cap_zar".to_string(),
                "Set to 0 for company/trust taxpayer class",
            ));
        }

        for (index, asset) in self.assets.iter().enumerate() {
            asset.validate_contract(
                index,
                self.taxpayer_class,
                self.residency_status,
                &mut issues,
            );
        }

        if issues.is_empty() {
            Ok(())
        } else {
            Err(InputValidationError::new(issues))
        }
    }
}
