# Estate Optimization Engine

Jurisdiction-aware estate-planning engine with:
- Combined Tax Liability calculation
- Liquidity Gap output
- Tax-rule version registry
- HTTP API (Cargo + Axum)

Current jurisdiction baselines:
- South Africa
- United States (state baselines: New York, Texas, California, Florida, Minnesota)

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
Tax baselines are maintained in:
- `jurisdictions/south_africa/mod.rs`
- `jurisdictions/us/mod.rs`
