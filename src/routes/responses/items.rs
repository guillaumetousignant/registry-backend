use crate::database::Item;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Items {
    pub data: Vec<Item>,
}
