use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::{Pool, PooledConnection};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

// TODO: This alias will facilitate the move to a more "production-ready" database (i.e.
// Postgre)
pub type DbConnection = SqliteConnection;
pub type DbPool = Pool<ConnectionManager<DbConnection>>;
pub type DbPoolConn = PooledConnection<ConnectionManager<DbConnection>>;

pub fn establish_connection() -> Result<DbPool, String> {
    let database_url = std::env::var("DB_CONNECTION_URL").map_err(|_| "Missing ${DB_CONNECTION_URL}")?;

    let manager = ConnectionManager::<DbConnection>::new(database_url);

    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .map_err(|_| "Could not create DB Pool")?;

    let mut conn = pool.get().map_err(|_| "Could not create DB connection")?;

    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|err| format!("Could not run migrations: {}", err))?;

    Ok(pool)
}
