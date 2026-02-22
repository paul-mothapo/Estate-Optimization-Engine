# South Africa Jurisdiction Overview

## Purpose
This jurisdiction package defines the South African legal and tax baseline used by the Estate Optimization Engine.  
It acts as the legal source of truth for South Africa so the core engine can call one jurisdiction interface and remain country-agnostic.

## What It Does
The South Africa module provides:
- A legal framework catalog for estate planning context.
- A baseline tax rule set for South Africa.
- A human-readable summary of key rates and thresholds used for planning scenarios.
- Reference links so rules can be reviewed and updated when legislation or SARS guidance changes.

In practical terms, this means scenario and optimizer layers can request South Africa rules and run deterministic calculations from one consistent rule source.

## Legal Instruments Captured
The module tracks the main legal instruments that shape estate planning outcomes in South Africa, including:
- Constitution of the Republic of South Africa.
- Wills Act.
- Intestate Succession Act.
- Administration of Estates Act.
- Estate Duty Act.
- Income Tax Act.
- Matrimonial Property Act.
- Trust Property Control Act.
- Pension Funds Act (including section 37C context).

These instruments matter because estate outcomes are not just tax outcomes. Validity of a will, marital property regime, trust administration, executor process, and retirement fund benefit allocation all affect how assets are ultimately distributed.

## Tax Baseline Modeled
The South Africa jurisdiction baseline currently models:
- Estate duty framework (including primary and secondary bands and abatement).
- Donations tax framework (including annual exemptions and rate bands).
- Capital gains treatment at death (including annual exclusion and inclusion-rate structure by taxpayer class).

This baseline is intended for planning simulations and comparative strategy scoring, not direct filing output.

## Tax Baseline Verification (As of 2026-02-21)
Latest verification date used for this package: `2026-02-21`.

Verified baseline values:
- Estate duty: 20% up to R30 million, 25% above R30 million, with section 4A abatement of R3.5 million.
- Donations tax: 20% up to R30 million cumulative taxable donations, 25% above R30 million.
- Donations exemptions: R100,000 annual exemption for natural persons; R10,000 for non-natural persons (casual gifts).
- CGT at death (individual/special trust): annual exclusion of R300,000 in year of death.
- CGT inclusion rates: 40% natural person/special trust, 80% company, 80% trust.

Primary sources:
- Estate duty: https://www.sars.gov.za/types-of-tax/estate-duty/
- Donations tax: https://www.sars.gov.za/types-of-tax/donations-tax/
- CGT rates and exclusions: https://www.sars.gov.za/tax-rates/income-tax/capital-gains-tax-cgt/

## Legalities and Compliance Boundaries
The module is designed with the following legal boundaries in mind:
- It supports planning logic, not legal advice.
- It does not replace an admitted attorney, tax practitioner, fiduciary specialist, or executor.
- It is a rule representation layer and cannot by itself validate full legal compliance in complex estates.
- It assumes user-provided inputs are complete and correct.
- It should be treated as jurisdiction-aware decision support, not an authoritative legal determination engine.

## Operational Assumptions
Current assumptions include:
- Baseline rates and thresholds are versioned by an effective period, but legislation can change.
- Certain deductions and exemptions may be simplified at baseline stage.
- Complex cross-border, trust-structure, and matrimonial edge cases may require dedicated extensions.

## Limitations
Known limits of this baseline:
- It does not yet cover every special-case exception in South African law.
- It does not yet model all procedural requirements of estate administration.
- It does not yet perform legal-document validity checks.

This provides a formal way to discover supported tax-year windows and version IDs used by the engine.

## Governance and Update Policy
To keep legal quality high:
- Jurisdiction files remain the source of truth for country-specific rules.
- Core tax logic should call into jurisdiction providers and avoid hardcoded country values.
- Any legal or tax update should include source verification and effective-date review.
- Changes should be logged with rationale so planners can trace why scenario outputs changed.

