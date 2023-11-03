use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

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

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.assigned {
            None => write!(
                f,
                "Item with id: {}, name: \"{}\", colour: \"{}\", link: \"{}\", unassigned",
                self.id, self.name, self.colour, self.link
            ),
            Some(assigned) => write!(
                f,
                "Item with id: {}, name: \"{}\", colour: \"{}\", link: \"{}\", assigned: \"{}\"",
                self.id, self.name, self.colour, self.link, assigned
            ),
        }
    }
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

impl fmt::Display for NewItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.assigned {
            None => write!(
                f,
                "New item with name: \"{}\", colour: \"{}\", link: \"{}\", unassigned",
                self.name, self.colour, self.link
            ),
            Some(assigned) => write!(
                f,
                "Item with name: \"{}\", colour: \"{}\", link: \"{}\", assigned: \"{}\"",
                self.name, self.colour, self.link, assigned
            ),
        }
    }
}
