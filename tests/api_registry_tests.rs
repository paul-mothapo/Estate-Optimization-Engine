use crate::api::handler::{
    get_jurisdiction_tax_rule_registry, list_supported_jurisdictions,
    list_tax_rule_registry_entries,
};
use crate::core::rules::tax_rules::Jurisdiction;

#[test]
fn api_lists_supported_jurisdictions() {
    let jurisdictions = list_supported_jurisdictions();
    assert_eq!(jurisdictions, vec![Jurisdiction::SouthAfrica]);
}

#[test]
fn api_lists_registry_entries() {
    let entries = list_tax_rule_registry_entries();
    assert!(entries.iter().any(|entry| {
        entry.jurisdiction == Jurisdiction::SouthAfrica
            && entry.version.version_id == "ZA-ESTATE-BASELINE-2018+"
    }));
}

#[test]
fn api_exposes_jurisdiction_registry_summary() {
    let summary = get_jurisdiction_tax_rule_registry(Jurisdiction::SouthAfrica)
        .expect("Expected registry summary for South Africa");

    assert_eq!(summary.jurisdiction, Jurisdiction::SouthAfrica);
    assert_eq!(summary.supported_tax_year_from, 2018);
    assert_eq!(summary.supported_tax_year_to, None);
    assert_eq!(summary.latest_version_id, "ZA-ESTATE-BASELINE-2018+");
    assert!(!summary.versions.is_empty());
}
