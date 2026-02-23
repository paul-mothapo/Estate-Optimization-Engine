mod routes;
mod state;

use axum::Router;
use sqlx::postgres::PgPoolOptions;

pub use state::AppState;

pub fn app() -> Router {
    app_with_state(AppState { db_pool: None })
}

pub fn app_with_state(state: AppState) -> Router {
    routes::router(state)
}

pub async fn run(
    bind_addr: &str,
    database_url: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let max_connections = std::env::var("DB_MAX_CONNECTIONS")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(10);

    let db_pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await?;

    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    axum::serve(
        listener,
        app_with_state(AppState {
            db_pool: Some(db_pool),
        }),
    )
    .await?;

    Ok(())
}
