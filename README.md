# NOTE
This follows:
A modular layered architecture with a domain-centric core and plug-in jurisdiction modules.

# Estate Optimization Engine

South Africa estate-planning engine with:
- Combined Tax Liability calculation
- Liquidity Gap output

## Run
Prerequisite:
- Rust installed (`rustc --version`)

Build executable:
```powershell
rustc --edition 2021 main.rs -o .\estate-engine.exe
```

Run executable:
```powershell
.\estate-engine.exe
```

Run tests:
```powershell
rustc --edition 2021 --test main.rs -o .\estate-engine-tests.exe
.\estate-engine-tests.exe
```

Tax baseline status:
- South Africa rates and thresholds were source-verified on `2026-02-21` in `jurisdictions/south_africa/mod.rs`.
