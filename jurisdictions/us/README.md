# US Jurisdiction Overview

## Purpose
This module provides a US state-level planning baseline for:
- New York
- Texas
- California
- Florida
- Minnesota

It is designed as a jurisdiction provider so core tax logic can remain country/state agnostic.

## Baseline Scope
The 2026+ baseline models:
- Estate transfer tax using a combined effective rate approach (federal + state overlay where applicable).
- Gift tax as a federal planning baseline.
- Capital gains at death using basis step-up treatment (no immediate CGT realization in this model).

## State Policy Mode
- New York: federal + NY state estate-tax overlay.
- Minnesota: federal + MN state estate-tax overlay.
- Texas/California/Florida: federal estate-tax baseline only.

## Legal/Operational Notes
- This is a planning engine baseline, not tax filing software.
- Rates/exemptions should be reviewed and versioned whenever federal or state rules change.
- Final advice and filing positions should be validated with licensed US tax counsel.
