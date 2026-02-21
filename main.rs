pub mod jurisdictions {
    #[path = "south-africa/south-africa.rs"]
    pub mod south_africa;
}

pub mod core {
    #[path = "tax-rules.rs"]
    pub mod tax_rules;
    #[path = "models.rs"]
    pub mod models;
    #[path = "scenario.rs"]
    pub mod scenario;
    #[path = "scoring.rs"]
    pub mod scoring;
    #[path = "optimizer.rs"]
    pub mod optimizer;
}

pub mod api {
    #[path = "handler.rs"]
    pub mod handler;
}

pub mod simulation {
    #[path = "monet-carlo.rs"]
    pub mod monet_carlo;
}

#[cfg(test)]
#[path = "tests/scenario_tests.rs"]
mod scenario_tests;

fn main() {}
