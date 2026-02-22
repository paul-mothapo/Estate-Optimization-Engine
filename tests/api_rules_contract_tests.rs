use crate::api::contracts::{ApiErrorCode, ApiJurisdiction};
use crate::api::handler::{
    get_jurisdiction_tax_rule_registry_contract, list_supported_jurisdictions_contract,
    list_tax_rule_registry_entries_contract, resolve_latest_tax_rules_contract,
    resolve_tax_rules_for_year_contract,
};

#[test]
fn contract_lists_supported_jurisdictions() {
    let jurisdictions = list_supported_jurisdictions_contract();
    assert_eq!(jurisdictions, vec![ApiJurisdiction::SouthAfrica]);
}

#[test]
fn contract_lists_registry_entries() {
    let entries = list_tax_rule_registry_entries_contract();
    assert!(entries.iter().any(|entry| {
        entry.jurisdiction == ApiJurisdiction::SouthAfrica
            && entry.version.version_id == "ZA-ESTATE-BASELINE-2018+"
    }));
}

#[test]
fn contract_exposes_jurisdiction_registry_summary() {
    let summary = get_jurisdiction_tax_rule_registry_contract(ApiJurisdiction::SouthAfrica)
        .expect("Expected contract registry summary for South Africa");

    assert_eq!(summary.jurisdiction, ApiJurisdiction::SouthAfrica);
    assert_eq!(summary.supported_tax_year_from, 2018);
    assert_eq!(summary.supported_tax_year_to, None);
    assert_eq!(summary.latest_version_id, "ZA-ESTATE-BASELINE-2018+");
    assert!(!summary.versions.is_empty());
}

#[test]
fn contract_resolves_rules_for_supported_year() {
    let selected = resolve_tax_rules_for_year_contract(ApiJurisdiction::SouthAfrica, 2026)
        .expect("Expected contract rule resolution for supported tax year");

    assert_eq!(selected.version.version_id, "ZA-ESTATE-BASELINE-2018+");
    assert_eq!(selected.version.tax_year_from, 2018);
    assert_eq!(selected.version.tax_year_to, None);
}

#[test]
fn contract_rejects_unsupported_year() {
    let err = resolve_tax_rules_for_year_contract(ApiJurisdiction::SouthAfrica, 2017)
        .expect_err("Expected contract rule resolution to fail");
    assert_eq!(err.code, ApiErrorCode::RuleSelection);
    assert!(err.message.contains("2017"));
}

#[test]
fn contract_resolves_latest_rules() {
    let latest = resolve_latest_tax_rules_contract(ApiJurisdiction::SouthAfrica);
    assert_eq!(latest.version.version_id, "ZA-ESTATE-BASELINE-2018+");
}
