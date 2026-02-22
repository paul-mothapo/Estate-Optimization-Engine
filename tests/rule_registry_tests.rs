use crate::core::rules::tax_rules::{
    latest_tax_rules_for, supported_jurisdictions, supported_tax_year_window, tax_rule_registry,
    tax_rule_registry_for, Jurisdiction,
};

#[test]
fn registry_exposes_south_africa_versions() {
    let versions = tax_rule_registry_for(Jurisdiction::SouthAfrica);
    assert!(!versions.is_empty());
    assert!(versions
        .iter()
        .any(|version| version.version_id == "ZA-ESTATE-BASELINE-2018+"));
}

#[test]
fn supported_jurisdictions_contains_south_africa() {
    let jurisdictions = supported_jurisdictions();
    assert_eq!(jurisdictions, vec![Jurisdiction::SouthAfrica]);
}

#[test]
fn tax_year_window_matches_catalog_bounds() {
    let window = supported_tax_year_window(Jurisdiction::SouthAfrica);
    assert_eq!(window, Some((2018, None)));
}

#[test]
fn latest_rule_version_is_present_in_registry() {
    let latest = latest_tax_rules_for(Jurisdiction::SouthAfrica);
    let registry = tax_rule_registry();
    assert!(registry.iter().any(|entry| {
        entry.jurisdiction == Jurisdiction::SouthAfrica
            && entry.version.version_id == latest.version.version_id
    }));
}
