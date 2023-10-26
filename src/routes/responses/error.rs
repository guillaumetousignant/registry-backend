use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub error: ErrorStruct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorStruct {
    pub message: String,
    pub code: i64,
}
