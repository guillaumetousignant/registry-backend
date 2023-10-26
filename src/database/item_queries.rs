use super::establish_connection;
use super::models::{Item, NewItem};
use super::DatabaseConnectionError;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;
use log::info;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseQueryError {
    #[error("database connection error")]
    Connection(#[from] DatabaseConnectionError),
    #[error("database query error")]
    Query(#[from] diesel::result::Error),
}

pub fn get_all_items() -> Result<Vec<Item>, DatabaseQueryError> {
    use crate::database::schema::items::dsl::*;

    let mut connection = establish_connection().map_err(|e| DatabaseQueryError::Connection(e))?;
    let results = items
        .select(Item::as_select())
        .load(&mut connection)
        .map_err(|e| DatabaseQueryError::Query(e))?;

    Ok(results)
}

pub fn insert_item(item: &NewItem) -> Result<(), DatabaseQueryError> {
    use crate::database::schema::items::dsl::*;

    let mut connection = establish_connection().map_err(|e| DatabaseQueryError::Connection(e))?;

    let count = diesel::insert_into(items)
        .values(item)
        .execute(&mut connection)
        .map_err(|e| DatabaseQueryError::Query(e))?;

    info!("Inserted {count} into items");

    Ok(())
}
