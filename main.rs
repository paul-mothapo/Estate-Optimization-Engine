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

#[cfg(test)]
#[path = "tests/rule_registry_tests.rs"]
mod rule_registry_tests;

#[cfg(test)]
#[path = "tests/api_registry_tests.rs"]
mod api_registry_tests;

fn main() {}
