#[path = "jurisdictions/mod.rs"]
pub mod jurisdictions;

#[path = "core/mod.rs"]
pub mod core;

#[path = "api/mod.rs"]
pub mod api;

#[path = "simulation/mod.rs"]
pub mod simulation;

#[cfg(test)]
#[path = "tests/scenario_tests.rs"]
mod scenario_tests;

#[cfg(test)]
#[path = "tests/validation_tests.rs"]
mod validation_tests;

fn main() {}
