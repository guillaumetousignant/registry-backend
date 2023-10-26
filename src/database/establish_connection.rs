use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use dotenvy::dotenv;
use std::env;
use std::env::VarError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseConnectionError {
    #[error("DATABASE_URL not set, either in environment variables or in .env file")]
    URLNotSet(#[from] VarError),
    #[error("database connection error")]
    Connection(#[from] diesel::ConnectionError),
}

pub fn establish_connection() -> Result<SqliteConnection, DatabaseConnectionError> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").map_err(|e| DatabaseConnectionError::URLNotSet(e))?;
    let connection = SqliteConnection::establish(&database_url)
        .map_err(|e| DatabaseConnectionError::Connection(e))?;
    Ok(connection)
}
