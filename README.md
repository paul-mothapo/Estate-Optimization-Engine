# Estate Optimization Engine

South Africa estate-planning engine with:
- Combined Tax Liability calculation
- Liquidity Gap output
- Tax-rule version registry
- HTTP API (Cargo + Axum)

## Stack
- Rust 2021
- Cargo
- Axum + Tokio

## Run
Prerequisites:
- Rust toolchain (`cargo --version`)

Start API server:
```bash
cargo run
```

Custom bind address:
```bash
ENGINE_BIND=0.0.0.0:8080 cargo run
```

Compile checks:
```bash
cargo check
cargo check --all-targets
```

## HTTP Endpoints
- `GET /health`
- `GET /v1/jurisdictions`
- `GET /v1/rules/registry`
- `GET /v1/rules/registry/{jurisdiction}`
- `GET /v1/rules/latest/{jurisdiction}`
- `GET /v1/rules/{jurisdiction}/{tax_year}`
- `POST /v1/scenario/calculate`
- `POST /v1/scenario/optimize`

Jurisdiction path values currently supported:
- `south-africa`
- `south_africa`
- `southafrica`
- `za`

## Verification
Tax baselines are maintained in `jurisdictions/south_africa/mod.rs` and currently source-verified as of `2026-02-21`.
