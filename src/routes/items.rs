use super::authorization::UserClaim;
use super::requests;
use super::responses;
use crate::database::NewItem;
use crate::database::{assign_item, get_all_items, insert_item, DatabaseQueryError};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[rocket::get("/")]
pub fn items(user: UserClaim) -> Result<Json<responses::Items>, (Status, Json<responses::Error>)> {
    match user.id.as_str() {
        "user" | "admin" => {
            let items = get_all_items().map_err(|e| {
                (
                    Status::InternalServerError,
                    Json(responses::Error {
                        error: responses::ErrorStruct {
                            message: e.to_string(),
                            code: 42,
                        },
                    }),
                )
            })?;

            Ok(Json(responses::Items { data: items }))
        }
        _ => Err((
            Status::Unauthorized,
            Json(responses::Error {
                error: responses::ErrorStruct {
                    message: "unauthorized".to_owned(),
                    code: 43,
                },
            }),
        )),
    }
}

#[rocket::post("/add", data = "<item>")]
pub fn add_item(
    user: UserClaim,
    item: Json<NewItem>,
) -> Result<status::Accepted<String>, (Status, Json<responses::Error>)> {
    match user.id.as_str() {
        "admin" => {
            insert_item(&item).map_err(|e| {
                (
                    Status::InternalServerError,
                    Json(responses::Error {
                        error: responses::ErrorStruct {
                            message: e.to_string(),
                            code: 44,
                        },
                    }),
                )
            })?;

            Ok(status::Accepted(None))
        }
        _ => Err((
            Status::Unauthorized,
            Json(responses::Error {
                error: responses::ErrorStruct {
                    message: "unauthorized".to_owned(),
                    code: 45,
                },
            }),
        )),
    }
}

#[rocket::post("/<id>/claim", data = "<assigned>")]
pub fn claim_item(
    id: i32,
    user: UserClaim,
    assigned: Json<requests::Assigned>,
) -> Result<status::Accepted<String>, (Status, Json<responses::Error>)> {
    match user.id.as_str() {
        "user" | "admin" => {
            assign_item(id, &assigned.assigned).map_err(|e| match e {
                DatabaseQueryError::NotUpdated => (
                    Status::Conflict,
                    Json(responses::Error {
                        error: responses::ErrorStruct {
                            message: e.to_string(),
                            code: 48,
                        },
                    }),
                ),
                _ => (
                    Status::InternalServerError,
                    Json(responses::Error {
                        error: responses::ErrorStruct {
                            message: e.to_string(),
                            code: 46,
                        },
                    }),
                ),
            })?;

            Ok(status::Accepted(None))
        }
        _ => Err((
            Status::Unauthorized,
            Json(responses::Error {
                error: responses::ErrorStruct {
                    message: "unauthorized".to_owned(),
                    code: 47,
                },
            }),
        )),
    }
}
