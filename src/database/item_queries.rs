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
    #[error("database select query error")]
    SelectQuery(#[source] diesel::result::Error),
    #[error("database insert query error")]
    InsertQuery(#[source] diesel::result::Error),
    #[error("database update query error")]
    UpdateQuery(#[source] diesel::result::Error),
    #[error("database delete query error")]
    DeleteQuery(#[source] diesel::result::Error),
    #[error("item not inserted")]
    NotInserted,
    #[error("item not updated")]
    NotUpdated,
    #[error("item not deleted")]
    NotDeleted,
    #[error("too many items inserted, expected one, found {0}")]
    TooManyInserted(usize),
    #[error("too many items updated, expected one, found {0}")]
    TooManyUpdated(usize),
    #[error("too many items deleted, expected one, found {0}")]
    TooManyDeleted(usize),
}

pub fn get_all_items() -> Result<Vec<Item>, DatabaseQueryError> {
    use crate::database::schema::items::dsl::*;

    let mut connection = establish_connection().map_err(|e| DatabaseQueryError::Connection(e))?;
    let results = items
        .select(Item::as_select())
        .load(&mut connection)
        .map_err(|e| DatabaseQueryError::SelectQuery(e))?;

    Ok(results)
}

pub fn insert_item(item: &NewItem) -> Result<(), DatabaseQueryError> {
    use crate::database::schema::items::dsl::*;

    let mut connection = establish_connection().map_err(|e| DatabaseQueryError::Connection(e))?;

    let count = diesel::insert_into(items)
        .values(item)
        .execute(&mut connection)
        .map_err(|e| DatabaseQueryError::InsertQuery(e))?;

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

    let count = diesel::update(items.filter(id.eq(item_id).and(assigned.is_null())))
        .set(assigned.eq(Some(assigned_user)))
        .execute(&mut connection)
        .map_err(|e| DatabaseQueryError::UpdateQuery(e))?;

    info!("Updated {count} item(s)");

    match count {
        0 => Err(DatabaseQueryError::NotUpdated),
        1 => Ok(()),
        _ => Err(DatabaseQueryError::TooManyUpdated(count)),
    }
}

pub fn delete_item(item_id: i32) -> Result<(), DatabaseQueryError> {
    use crate::database::schema::items::dsl::*;

    let mut connection = establish_connection().map_err(|e| DatabaseQueryError::Connection(e))?;

    let count = diesel::delete(items.filter(id.eq(item_id)))
        .execute(&mut connection)
        .map_err(|e| DatabaseQueryError::DeleteQuery(e))?;

    info!("Deleted {count} item(s)");

    match count {
        0 => Err(DatabaseQueryError::NotDeleted),
        1 => Ok(()),
        _ => Err(DatabaseQueryError::TooManyDeleted(count)),
    }
}

pub fn unassign_item(item_id: i32) -> Result<(), DatabaseQueryError> {
    use crate::database::schema::items::dsl::*;

    let mut connection = establish_connection().map_err(|e| DatabaseQueryError::Connection(e))?;

    let count = diesel::update(items.filter(id.eq(item_id).and(assigned.is_not_null())))
        .set(assigned.eq(None::<&str>))
        .execute(&mut connection)
        .map_err(|e| DatabaseQueryError::UpdateQuery(e))?;

    info!("Unassigned {count} item(s)");

    match count {
        0 => Err(DatabaseQueryError::NotUpdated),
        1 => Ok(()),
        _ => Err(DatabaseQueryError::TooManyUpdated(count)),
    }
}

pub fn update_item_link(item_id: i32, new_link: &str) -> Result<(), DatabaseQueryError> {
    use crate::database::schema::items::dsl::*;

    let mut connection = establish_connection().map_err(|e| DatabaseQueryError::Connection(e))?;

    let count = diesel::update(items.filter(id.eq(item_id)))
        .set(link.eq(new_link))
        .execute(&mut connection)
        .map_err(|e| DatabaseQueryError::UpdateQuery(e))?;

    info!("Updated link of {count} item(s)");

    match count {
        0 => Err(DatabaseQueryError::NotUpdated),
        1 => Ok(()),
        _ => Err(DatabaseQueryError::TooManyUpdated(count)),
    }
}
