use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::database::schema::items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub colour: String,
    pub link: String,
    pub assigned: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::database::schema::items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewItem {
    pub name: String,
    pub colour: String,
    pub link: String,
    pub assigned: Option<String>,
}
