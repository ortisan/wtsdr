use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, r2d2};

pub struct DbConfig {
    pub(crate) database_url: String,
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_connection(url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[derive(Debug, Clone)]
pub struct PostgresBaseRepository {
    pub pool: DbPool,
}

impl PostgresBaseRepository {
    pub fn new(db_config: DbConfig) -> Self {
        let pool = get_connection(&db_config.database_url);
        PostgresBaseRepository { pool }
    }
}
