use crate::api::handler::{resolve_latest_tax_rules, resolve_tax_rules_for_year};
use crate::core::errors::EngineError;
use crate::core::rules::tax_rules::Jurisdiction;

#[test]
fn api_resolves_tax_rules_for_supported_year() {
    let selected = resolve_tax_rules_for_year(Jurisdiction::SouthAfrica, 2026)
        .expect("Expected supported tax year to resolve");

    assert_eq!(selected.version.version_id, "ZA-ESTATE-BASELINE-2018+");
    assert_eq!(selected.version.tax_year_from, 2018);
    assert_eq!(selected.version.tax_year_to, None);
}

#[test]
fn api_rejects_unsupported_tax_year() {
    let err = resolve_tax_rules_for_year(Jurisdiction::SouthAfrica, 2017)
        .expect_err("Expected unsupported tax year to fail");

    let EngineError::RuleSelection(selection_error) = err else {
        panic!("Expected rule selection error");
    };

    let rendered = selection_error.to_string();
    assert!(rendered.contains("SouthAfrica"));
    assert!(rendered.contains("2017"));
}

#[test]
fn api_resolves_latest_tax_rules() {
    let latest = resolve_latest_tax_rules(Jurisdiction::SouthAfrica);
    assert_eq!(latest.version.version_id, "ZA-ESTATE-BASELINE-2018+");
}
