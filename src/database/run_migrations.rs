use super::{establish_connection, DatabaseConnectionError};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use thiserror::Error;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("database connection error")]
    Connection(#[from] DatabaseConnectionError),
    #[error("database migration error")]
    Migration(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub fn run_migrations() -> Result<(), MigrationError> {
    let mut connection = establish_connection().map_err(|e| MigrationError::Connection(e))?;

    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| MigrationError::Migration(e))?;

    Ok(())
}
