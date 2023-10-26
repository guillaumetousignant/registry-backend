use super::establish_connection;
use super::models::{Item, NewItem};
use super::DatabaseConnectionError;
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
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
    #[error("item not inserted")]
    NotInserted,
    #[error("item not updated")]
    NotUpdated,
    #[error("too many items inserted, expected one, found {0}")]
    TooManyInserted(usize),
    #[error("too many items updated, expected one, found {0}")]
    TooManyUpdated(usize),
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

    info!("Inserted {count} into item(s)");

    match count {
        0 => Err(DatabaseQueryError::NotInserted),
        1 => Ok(()),
        _ => Err(DatabaseQueryError::TooManyInserted(count)),
    }
}

pub fn assign_item(item_id: i32, assigned_user: &str) -> Result<(), DatabaseQueryError> {
    use crate::database::schema::items::dsl::*;

    let mut connection = establish_connection().map_err(|e| DatabaseQueryError::Connection(e))?;

    let count =
        diesel::update(items.filter(id.eq(item_id).and(assigned.eq(Option::<String>::None))))
            .set(assigned.eq(Some(assigned_user)))
            .execute(&mut connection)
            .map_err(|e| DatabaseQueryError::Query(e))?;

    info!("Updated {count} item(s)");

    match count {
        0 => Err(DatabaseQueryError::NotUpdated),
        1 => Ok(()),
        _ => Err(DatabaseQueryError::TooManyUpdated(count)),
    }
}
