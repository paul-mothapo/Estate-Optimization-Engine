use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_pool: Option<PgPool>,
}
