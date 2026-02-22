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

fn resolve_bind_addr() -> String {
    if let Ok(bind) = std::env::var("ENGINE_BIND") {
        if !bind.trim().is_empty() {
            return bind;
        }
    }

    if let Ok(port) = std::env::var("PORT") {
        let trimmed = port.trim();
        if !trimmed.is_empty() {
            return format!("0.0.0.0:{trimmed}");
        }
    }

    "127.0.0.1:8080".to_string()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let _ = dotenvy::dotenv();
    let bind_addr = resolve_bind_addr();
    api::run(&bind_addr).await
}
