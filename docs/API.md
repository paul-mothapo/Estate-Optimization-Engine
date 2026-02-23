# API Documentation

## Overview
- Base URL: `http://127.0.0.1:8080` (default)
- Content type: `application/json`
- Auth: none
- Current jurisdiction support: South Africa

## Run
Required environment variables:
- `DATABASE_URL` (PostgreSQL connection string)

Optional environment variables:
- `ENGINE_BIND` (example: `0.0.0.0:8080`)
- `PORT` (used when `ENGINE_BIND` is not set)
- `DB_MAX_CONNECTIONS` (default: `10`)

Start server:
```bash
cargo run
```

## Error Contract
All non-2xx responses return:

```json
{
  "code": "Validation | RuleSelection | Computation",
  "message": "string",
  "validation_issues": [
    {
      "field": "string",
      "message": "string"
    }
  ]
}
```

HTTP status mapping:
- `Validation` -> `400 Bad Request`
- `RuleSelection` -> `422 Unprocessable Entity`
- `Computation` -> `500 Internal Server Error` (or `503 Service Unavailable` for health checks)

## Jurisdiction Path Values
These path values are accepted anywhere `{jurisdiction}` is used:
- `south-africa`
- `south_africa`
- `southafrica`
- `za`

## Endpoints

### `GET /health`
Returns API liveness.

Example response (`200`):
```json
{
  "status": "ok"
}
```

### `GET /health/db`
Returns database health.

Example response (`200`):
```json
{
  "status": "ok"
}
```

Example response when DB is unavailable (`503`):
```json
{
  "code": "Computation",
  "message": "Database health check failed: ...",
  "validation_issues": []
}
```

### `GET /v1/jurisdictions`
Lists supported jurisdictions.

Example response (`200`):
```json
[
  "SouthAfrica"
]
```

### `GET /v1/rules/registry`
Lists all tax-rule versions across supported jurisdictions.

Example response (`200`):
```json
[
  {
    "jurisdiction": "SouthAfrica",
    "version": {
      "version_id": "ZA-ESTATE-BASELINE-2018+",
      "tax_year_from": 2018,
      "tax_year_to": null,
      "effective_from": "2018-03-01",
      "effective_to": null,
      "source_last_verified_on": "2026-02-21"
    }
  }
]
```

### `GET /v1/rules/registry/{jurisdiction}`
Returns rule-version registry summary for a jurisdiction.

Example response (`200`):
```json
{
  "jurisdiction": "SouthAfrica",
  "versions": [
    {
      "version_id": "ZA-ESTATE-BASELINE-2018+",
      "tax_year_from": 2018,
      "tax_year_to": null,
      "effective_from": "2018-03-01",
      "effective_to": null,
      "source_last_verified_on": "2026-02-21"
    }
  ],
  "supported_tax_year_from": 2018,
  "supported_tax_year_to": null,
  "latest_version_id": "ZA-ESTATE-BASELINE-2018+"
}
```

### `GET /v1/rules/latest/{jurisdiction}`
Returns the latest versioned tax rules for a jurisdiction.

Example response (`200`):
```json
{
  "version": {
    "version_id": "ZA-ESTATE-BASELINE-2018+",
    "tax_year_from": 2018,
    "tax_year_to": null,
    "effective_from": "2018-03-01",
    "effective_to": null,
    "source_last_verified_on": "2026-02-21"
  },
  "rules": {
    "estate_duty": {
      "section_4a_abatement_zar": 3500000.0,
      "primary_rate": 0.2,
      "primary_rate_cap_zar": 30000000.0,
      "secondary_rate": 0.25,
      "spouse_deduction_unlimited": true,
      "effective_from": "2018-03-01",
      "source": "Estate Duty Act 45 of 1955",
      "source_url": "..."
    },
    "donations_tax": {
      "annual_exemption_natural_person_zar": 100000.0,
      "annual_exemption_non_natural_casual_gifts_zar": 10000.0,
      "primary_rate": 0.2,
      "primary_rate_cap_cumulative_zar": 30000000.0,
      "secondary_rate": 0.25,
      "effective_from": "2018-03-01",
      "source": "Income Tax Act 58 of 1962",
      "source_url": "..."
    },
    "cgt_on_death": {
      "annual_exclusion_in_year_of_death_zar": 300000.0,
      "inclusion_rate_natural_person": 0.4,
      "inclusion_rate_company": 0.8,
      "inclusion_rate_trust": 0.8,
      "base_cost_step_up_to_market_value_on_death": true,
      "effective_from": "2016-03-01",
      "source": "Income Tax Act 58 of 1962",
      "source_url": "..."
    }
  }
}
```

### `GET /v1/rules/{jurisdiction}/{tax_year}`
Returns tax rules for a specific tax year.

Example:
```bash
curl http://127.0.0.1:8080/v1/rules/south-africa/2026
```

Error example for unsupported year (`422`):
```json
{
  "code": "RuleSelection",
  "message": "No tax rule version found for jurisdiction SouthAfrica and tax year 2017",
  "validation_issues": []
}
```

### `POST /v1/scenario/calculate`
Calculates combined tax and liquidity outputs for one scenario.

Request body:
```json
{
  "jurisdiction": "SouthAfrica",
  "tax_year": 2026,
  "taxpayer_class": "NaturalPerson",
  "residency_status": "Resident",
  "marginal_income_tax_rate": 0.45,
  "assets": [
    {
      "name": "Primary Residence",
      "market_value_zar": 5000000.0,
      "base_cost_zar": 2000000.0,
      "is_liquid": false,
      "situs_in_south_africa": true,
      "included_in_estate_duty": true,
      "included_in_cgt_deemed_disposal": true,
      "bequeathed_to_surviving_spouse": false,
      "bequeathed_to_pbo": false,
      "qualifies_primary_residence_exclusion": true
    }
  ],
  "debts_and_loans_zar": 250000.0,
  "funeral_costs_zar": 50000.0,
  "administration_costs_zar": 35000.0,
  "masters_office_fees_zar": 7000.0,
  "conveyancing_costs_zar": 25000.0,
  "other_settlement_costs_zar": 10000.0,
  "final_income_tax_due_zar": 120000.0,
  "ongoing_estate_income_tax_provision_zar": 15000.0,
  "additional_allowable_estate_duty_deductions_zar": 0.0,
  "ported_section_4a_abatement_zar": 0.0,
  "primary_residence_cgt_exclusion_cap_zar": 2000000.0,
  "executor_fee_rate": 0.035,
  "vat_rate": 0.15,
  "explicit_executor_fee_zar": null,
  "external_liquidity_proceeds_zar": 300000.0,
  "cash_reserve_zar": 200000.0
}
```

Response body (`200`):
```json
{
  "cgt": {
    "gross_capital_gain_zar": 3000000.0,
    "primary_residence_exclusion_used_zar": 2000000.0,
    "annual_exclusion_used_zar": 300000.0,
    "inclusion_rate": 0.4,
    "taxable_capital_gain_in_income_zar": 280000.0,
    "tax_payable_zar": 126000.0
  },
  "estate_duty": {
    "gross_estate_for_estate_duty_zar": 5000000.0,
    "executor_fee_zar": 201250.0,
    "section_4q_spousal_deduction_zar": 0.0,
    "pbo_deduction_zar": 0.0,
    "total_allowable_deductions_zar": 528250.0,
    "section_4a_abatement_used_zar": 3500000.0,
    "dutiable_estate_after_abatement_zar": 971750.0,
    "tax_payable_zar": 194350.0
  },
  "combined_tax": {
    "estate_duty_zar": 194350.0,
    "cgt_on_death_zar": 126000.0,
    "final_income_tax_zar": 120000.0,
    "ongoing_estate_income_tax_provision_zar": 15000.0,
    "total_tax_liability_zar": 455350.0
  },
  "liquidity": {
    "liquid_assets_in_estate_zar": 0.0,
    "external_liquidity_proceeds_zar": 300000.0,
    "cash_reserve_zar": 200000.0,
    "total_available_liquidity_zar": 500000.0,
    "executor_fee_zar": 201250.0,
    "immediate_cash_requirements_zar": 656600.0,
    "liquidity_gap_zar": 156600.0,
    "liquidity_surplus_zar": 0.0
  }
}
```

### `POST /v1/scenario/optimize`
Selects the best scenario from a list, based on composite score.

Request body:
```json
[
  {
    "jurisdiction": "SouthAfrica",
    "tax_year": 2026,
    "taxpayer_class": "NaturalPerson",
    "residency_status": "Resident",
    "marginal_income_tax_rate": 0.45,
    "assets": [
      {
        "name": "Asset A",
        "market_value_zar": 1000000.0,
        "base_cost_zar": 700000.0,
        "is_liquid": true,
        "situs_in_south_africa": true,
        "included_in_estate_duty": true,
        "included_in_cgt_deemed_disposal": true,
        "bequeathed_to_surviving_spouse": false,
        "bequeathed_to_pbo": false,
        "qualifies_primary_residence_exclusion": false
      }
    ],
    "debts_and_loans_zar": 0.0,
    "funeral_costs_zar": 0.0,
    "administration_costs_zar": 0.0,
    "masters_office_fees_zar": 0.0,
    "conveyancing_costs_zar": 0.0,
    "other_settlement_costs_zar": 0.0,
    "final_income_tax_due_zar": 0.0,
    "ongoing_estate_income_tax_provision_zar": 0.0,
    "additional_allowable_estate_duty_deductions_zar": 0.0,
    "ported_section_4a_abatement_zar": 0.0,
    "primary_residence_cgt_exclusion_cap_zar": 2000000.0,
    "executor_fee_rate": 0.035,
    "vat_rate": 0.15,
    "explicit_executor_fee_zar": 0.0,
    "external_liquidity_proceeds_zar": 0.0,
    "cash_reserve_zar": 0.0
  }
]
```

Response body (`200`):
```json
{
  "index": 0,
  "input": {
    "jurisdiction": "SouthAfrica",
    "tax_year": 2026,
    "taxpayer_class": "NaturalPerson",
    "residency_status": "Resident",
    "marginal_income_tax_rate": 0.45,
    "assets": [
      {
        "name": "Asset A",
        "market_value_zar": 1000000.0,
        "base_cost_zar": 700000.0,
        "is_liquid": true,
        "situs_in_south_africa": true,
        "included_in_estate_duty": true,
        "included_in_cgt_deemed_disposal": true,
        "bequeathed_to_surviving_spouse": false,
        "bequeathed_to_pbo": false,
        "qualifies_primary_residence_exclusion": false
      }
    ],
    "debts_and_loans_zar": 0.0,
    "funeral_costs_zar": 0.0,
    "administration_costs_zar": 0.0,
    "masters_office_fees_zar": 0.0,
    "conveyancing_costs_zar": 0.0,
    "other_settlement_costs_zar": 0.0,
    "final_income_tax_due_zar": 0.0,
    "ongoing_estate_income_tax_provision_zar": 0.0,
    "additional_allowable_estate_duty_deductions_zar": 0.0,
    "ported_section_4a_abatement_zar": 0.0,
    "primary_residence_cgt_exclusion_cap_zar": 2000000.0,
    "executor_fee_rate": 0.035,
    "vat_rate": 0.15,
    "explicit_executor_fee_zar": 0.0,
    "external_liquidity_proceeds_zar": 0.0,
    "cash_reserve_zar": 0.0
  },
  "result": {
    "cgt": {
      "gross_capital_gain_zar": 300000.0,
      "primary_residence_exclusion_used_zar": 0.0,
      "annual_exclusion_used_zar": 300000.0,
      "inclusion_rate": 0.4,
      "taxable_capital_gain_in_income_zar": 0.0,
      "tax_payable_zar": 0.0
    },
    "estate_duty": {
      "gross_estate_for_estate_duty_zar": 1000000.0,
      "executor_fee_zar": 40250.0,
      "section_4q_spousal_deduction_zar": 0.0,
      "pbo_deduction_zar": 0.0,
      "total_allowable_deductions_zar": 40250.0,
      "section_4a_abatement_used_zar": 959750.0,
      "dutiable_estate_after_abatement_zar": 0.0,
      "tax_payable_zar": 0.0
    },
    "combined_tax": {
      "estate_duty_zar": 0.0,
      "cgt_on_death_zar": 0.0,
      "final_income_tax_zar": 0.0,
      "ongoing_estate_income_tax_provision_zar": 0.0,
      "total_tax_liability_zar": 0.0
    },
    "liquidity": {
      "liquid_assets_in_estate_zar": 1000000.0,
      "external_liquidity_proceeds_zar": 0.0,
      "cash_reserve_zar": 0.0,
      "total_available_liquidity_zar": 1000000.0,
      "executor_fee_zar": 40250.0,
      "immediate_cash_requirements_zar": 40250.0,
      "liquidity_gap_zar": 0.0,
      "liquidity_surplus_zar": 959750.0
    }
  },
  "score": {
    "tax_burden_ratio": 0.0,
    "liquidity_cover_ratio": 24.84472049689441,
    "liquidity_risk_band": "Low",
    "composite_score": 0.0
  }
}
```

If no candidates are provided, response is `null`.

## Validation Rules (Summary)
- `assets` must contain at least one item, and at least one asset must have `market_value_zar > 0`.
- Rate fields must be within `0.0..=1.0`: `marginal_income_tax_rate`, `executor_fee_rate`, `vat_rate`.
- Monetary fields must be finite and non-negative.
- `tax_year` must be supported by the selected jurisdiction.
- Asset-level constraints:
  - `name` cannot be empty.
  - Cannot be bequeathed to both spouse and PBO.
  - Spouse/PBO flags require `included_in_estate_duty=true`.
  - `qualifies_primary_residence_exclusion=true` requires `included_in_cgt_deemed_disposal=true`.
  - For `Company`/`Trust`, set `primary_residence_cgt_exclusion_cap_zar=0` and do not flag primary residence exclusion on assets.
  - For `NonResident`, estate-duty-included assets must have `situs_in_south_africa=true`.
