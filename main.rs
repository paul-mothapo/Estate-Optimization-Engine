#[path = "jurisdictions/mod.rs"]
pub mod jurisdictions;

#[path = "core/mod.rs"]
pub mod core;

#[path = "api/mod.rs"]
pub mod api;

#[path = "simulation/mod.rs"]
pub mod simulation;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;

fn main() {}
