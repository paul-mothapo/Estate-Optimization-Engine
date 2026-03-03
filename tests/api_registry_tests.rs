use crate::api::handler::{
    get_jurisdiction_tax_rule_registry, list_supported_jurisdictions,
    list_tax_rule_registry_entries,
};
use crate::core::rules::tax_rules::Jurisdiction;

#[test]
fn api_lists_supported_jurisdictions() {
    let jurisdictions = list_supported_jurisdictions();
    assert_eq!(
        jurisdictions,
        vec![
            Jurisdiction::SouthAfrica,
            Jurisdiction::UsNewYork,
            Jurisdiction::UsTexas,
            Jurisdiction::UsCalifornia,
            Jurisdiction::UsFlorida,
            Jurisdiction::UsMinnesota,
        ]
    );
}

#[test]
fn api_lists_registry_entries() {
    let entries = list_tax_rule_registry_entries();
    assert!(entries.iter().any(|entry| {
        entry.jurisdiction == Jurisdiction::SouthAfrica
            && entry.version.version_id == "ZA-ESTATE-BASELINE-2018+"
    }));
    assert!(entries
        .iter()
        .any(|entry| entry.jurisdiction == Jurisdiction::UsNewYork));
    assert!(entries
        .iter()
        .any(|entry| entry.jurisdiction == Jurisdiction::UsTexas));
    assert!(entries
        .iter()
        .any(|entry| entry.jurisdiction == Jurisdiction::UsCalifornia));
    assert!(entries
        .iter()
        .any(|entry| entry.jurisdiction == Jurisdiction::UsFlorida));
    assert!(entries
        .iter()
        .any(|entry| entry.jurisdiction == Jurisdiction::UsMinnesota));
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

#[test]
fn api_exposes_us_state_registry_summary() {
    let summary = get_jurisdiction_tax_rule_registry(Jurisdiction::UsNewYork)
        .expect("Expected registry summary for US New York");

    assert_eq!(summary.jurisdiction, Jurisdiction::UsNewYork);
    assert_eq!(summary.supported_tax_year_from, 2026);
    assert_eq!(summary.supported_tax_year_to, None);
    assert_eq!(summary.latest_version_id, "US-NY-ESTATE-BASELINE-2026+");
    assert!(!summary.versions.is_empty());
}
