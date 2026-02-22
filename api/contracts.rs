#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JurisdictionTaxRuleRegistryResponse {
    pub jurisdiction: crate::core::rules::tax_rules::Jurisdiction,
    pub versions: Vec<crate::core::rules::tax_rules::TaxRuleVersion>,
    pub supported_tax_year_from: u16,
    pub supported_tax_year_to: Option<u16>,
    pub latest_version_id: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiErrorCode {
    Validation,
    RuleSelection,
    Computation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiValidationIssue {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiErrorResponse {
    pub code: ApiErrorCode,
    pub message: String,
    pub validation_issues: Vec<ApiValidationIssue>,
}
