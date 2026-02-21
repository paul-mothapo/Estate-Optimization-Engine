# Estate Optimization Engine

## Overview
Estate Optimization Engine is a jurisdiction-aware planning platform for evaluating wealth-transfer strategies.
It is designed to support predictive analytics, scenario modeling, and tax-aware optimization across multiple countries.

The core design principle is:
- jurisdiction modules are the legal and tax source of truth
- core modules provide reusable types, orchestration, and calculation entry points

## Current Scope
The repository is in early scaffold stage and currently includes a South Africa baseline jurisdiction package.

## Jurisdiction-First Rule Model
The rule flow is intentionally one-directional:
1. A jurisdiction module defines legal/tax baselines.
2. Core tax rules dispatch to the jurisdiction provider.
3. Scenario and optimizer layers consume the dispatched rules.

This avoids country-specific constants in core logic and makes adding new jurisdictions predictable.

## Combined Tax Liability Calculator
The engine now calculates a single combined tax result for a South African estate scenario by aggregating:
1. Estate duty
2. CGT on deemed disposal at death
3. Final income tax due
4. Ongoing estate income tax provision

Key South African rules currently applied in this flow:
- Estate duty primary and secondary rates with Section 4A abatement
- Section 4(q) spouse deduction treatment
- PBO bequest deduction support
- CGT inclusion-rate handling by taxpayer class
- Primary residence and annual exclusion handling for CGT inputs

## Liquidity Gap Output
The engine now calculates liquidity sufficiency for settlement costs and taxes.

Formula:
- `liquidity_gap = max(0, immediate_cash_requirements - total_available_liquidity)`
- `liquidity_surplus = max(0, total_available_liquidity - immediate_cash_requirements)`

Immediate cash requirements include:
- Combined tax liability
- Debts and loans
- Funeral and administration costs
- Executor fee
- Master's Office and conveyancing costs
- Other settlement costs

Available liquidity includes:
- Liquid assets inside the estate
- External liquidity proceeds
- Cash reserve

Output is available in `ScenarioResult.liquidity` with fields for:
- Available liquidity totals
- Immediate cash requirement total
- Liquidity gap
- Liquidity surplus

## Legal Positioning
This project is a decision-support engine for planning analysis.
It does not replace legal or tax advice and is not filing software.
Production use requires professional review by qualified legal and tax practitioners.
