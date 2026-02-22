use crate::config::Config;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::sync::Arc;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
        let pool = r2d2::Pool::builder().build(manager)?;

        Ok(Self {
            db_pool: pool,
            config: Arc::new(config),
        })
    }
}
