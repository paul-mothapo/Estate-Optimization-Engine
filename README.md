# Estate Optimization Engine

## Overview
Estate Optimization Engine is a jurisdiction-aware planning platform for evaluating wealth-transfer strategies.
It is designed to support predictive analytics, scenario modeling, and tax-aware optimization across multiple countries.

The core design principle is:
- jurisdiction modules are the legal and tax source of truth
- core modules provide reusable types, orchestration, and calculation entry points

## Current Scope
The repository is in early scaffold stage and currently includes a South Africa baseline jurisdiction package.

Implemented foundation:
- Jurisdiction-dispatch tax API in `core/tax-rules.rs`
- South Africa legal and tax baseline in `jurisdictions/south-africa/south-africa.rs`
- Human-readable legal context in `jurisdictions/south-africa/README.md`

## Repository Structure
- `api/`  
  API handlers and service boundaries.
- `core/`  
  Shared domain models, scenario engine, scoring, optimization, and jurisdiction call interfaces.
- `jurisdictions/`  
  Country-specific legal and tax rule providers. This is the source-of-truth layer for local laws.
- `simulation/`  
  Monte Carlo and related simulation workflows.
- `main.rs`  
  Application entry point.

## Jurisdiction-First Rule Model
The rule flow is intentionally one-directional:
1. A jurisdiction module defines legal/tax baselines.
2. Core tax rules dispatch to the jurisdiction provider.
3. Scenario and optimizer layers consume the dispatched rules.

This avoids country-specific constants in core logic and makes adding new jurisdictions predictable.

## Legal Positioning
This project is a decision-support engine for planning analysis.
It does not replace legal or tax advice and is not filing software.
Production use requires professional review by qualified legal and tax practitioners.

## Roadmap
Near-term implementation priorities:
1. Build deterministic scenario calculations in `core/scenario.rs`
2. Implement scoring in `core/scoring.rs`
3. Add baseline optimization strategies in `core/optimizer.rs`
4. Add simulation flows in `simulation/`
5. Add test coverage and tax-year versioning for jurisdiction rules

## Contribution Guidelines (Initial)
- Keep jurisdiction-specific values inside `jurisdictions/`
- Keep `core/` country-agnostic
- Include legal/tax source links and effective dates for any rule updates
- Document assumptions and limitations for each jurisdiction package
