use std::sync::Mutex;

use sqlx::PgPool;

pub struct AppState {
    pub vis_count: Mutex<u32>,
    pub db: PgPool,
}