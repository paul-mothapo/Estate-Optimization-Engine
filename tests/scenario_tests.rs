use crate::core::domain::models::{EstateAsset, EstateScenarioInput, ResidencyStatus};
use crate::core::engine::scenario::calculate_combined_tax_and_liquidity;
use crate::core::rules::tax_rules::{Jurisdiction, TaxPayerClass};

fn baseline_input() -> EstateScenarioInput {
    EstateScenarioInput {
        jurisdiction: Jurisdiction::SouthAfrica,
        taxpayer_class: TaxPayerClass::NaturalPerson,
        residency_status: ResidencyStatus::Resident,
        marginal_income_tax_rate: 0.45,
        explicit_executor_fee_zar: Some(0.0),
        ..EstateScenarioInput::default()
    }
}

#[test]
fn applies_estate_duty_primary_and_secondary_bands() {
    let mut input = baseline_input();
    input.assets.push(EstateAsset {
        name: "Illiquid asset".to_string(),
        market_value_zar: 40_000_000.0,
        base_cost_zar: 40_000_000.0,
        is_liquid: false,
        situs_in_south_africa: true,
        included_in_estate_duty: true,
        included_in_cgt_deemed_disposal: false,
        bequeathed_to_surviving_spouse: false,
        bequeathed_to_pbo: false,
        qualifies_primary_residence_exclusion: false,
    });

    let result = calculate_combined_tax_and_liquidity(&input);
    assert!((result.estate_duty.tax_payable_zar - 7_625_000.0).abs() < 0.1);
    assert!((result.liquidity.liquidity_gap_zar - 7_625_000.0).abs() < 0.1);
}

#[test]
fn applies_section_4q_spousal_deduction() {
    let mut input = baseline_input();
    input.assets.extend([
        EstateAsset {
            name: "Spouse bequest".to_string(),
            market_value_zar: 20_000_000.0,
            base_cost_zar: 20_000_000.0,
            is_liquid: false,
            situs_in_south_africa: true,
            included_in_estate_duty: true,
            included_in_cgt_deemed_disposal: false,
            bequeathed_to_surviving_spouse: true,
            bequeathed_to_pbo: false,
            qualifies_primary_residence_exclusion: false,
        },
        EstateAsset {
            name: "Non-spouse bequest".to_string(),
            market_value_zar: 5_000_000.0,
            base_cost_zar: 5_000_000.0,
            is_liquid: false,
            situs_in_south_africa: true,
            included_in_estate_duty: true,
            included_in_cgt_deemed_disposal: false,
            bequeathed_to_surviving_spouse: false,
            bequeathed_to_pbo: false,
            qualifies_primary_residence_exclusion: false,
        },
    ]);

    let result = calculate_combined_tax_and_liquidity(&input);
    assert!((result.estate_duty.section_4q_spousal_deduction_zar - 20_000_000.0).abs() < 0.1);
    assert!((result.estate_duty.tax_payable_zar - 300_000.0).abs() < 0.1);
}

#[test]
fn excludes_foreign_assets_for_non_resident_estate_duty_scope() {
    let mut input = baseline_input();
    input.residency_status = ResidencyStatus::NonResident;
    input.assets.extend([
        EstateAsset {
            name: "SA situs asset".to_string(),
            market_value_zar: 10_000_000.0,
            base_cost_zar: 10_000_000.0,
            is_liquid: false,
            situs_in_south_africa: true,
            included_in_estate_duty: true,
            included_in_cgt_deemed_disposal: false,
            bequeathed_to_surviving_spouse: false,
            bequeathed_to_pbo: false,
            qualifies_primary_residence_exclusion: false,
        },
        EstateAsset {
            name: "Foreign situs asset".to_string(),
            market_value_zar: 50_000_000.0,
            base_cost_zar: 50_000_000.0,
            is_liquid: false,
            situs_in_south_africa: false,
            included_in_estate_duty: true,
            included_in_cgt_deemed_disposal: false,
            bequeathed_to_surviving_spouse: false,
            bequeathed_to_pbo: false,
            qualifies_primary_residence_exclusion: false,
        },
    ]);

    let result = calculate_combined_tax_and_liquidity(&input);
    assert!((result.estate_duty.gross_estate_for_estate_duty_zar - 10_000_000.0).abs() < 0.1);
}
