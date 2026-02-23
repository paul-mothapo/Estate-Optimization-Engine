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

## Verification
Tax baselines are maintained in `jurisdictions/south_africa/mod.rs` and currently source-verified as of `2026-02-21`.
