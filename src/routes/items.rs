use super::authorization::UserClaim;
use super::requests;
use super::responses;
use crate::database::NewItem;
use crate::database::{
    assign_item, delete_item, get_all_items, insert_item, unassign_item, update_item_link,
    DatabaseQueryError,
};
use log::info;
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

            info!("Sent {} items", items.len());

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

            info!("Added item {}", item.0);

            Ok(status::Accepted("Added".to_owned()))
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

            info!("Claimed item {id} by {}", assigned.assigned);

            Ok(status::Accepted("Claimed".to_owned()))
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

#[rocket::post("/<id>/delete")]
pub fn remove_item(
    id: i32,
    user: UserClaim,
) -> Result<status::Accepted<String>, (Status, Json<responses::Error>)> {
    match user.id.as_str() {
        "admin" => {
            delete_item(id).map_err(|e| {
                (
                    Status::InternalServerError,
                    Json(responses::Error {
                        error: responses::ErrorStruct {
                            message: e.to_string(),
                            code: 49,
                        },
                    }),
                )
            })?;

            info!("Deleted item {id}");

            Ok(status::Accepted("Deleted".to_owned()))
        }
        _ => Err((
            Status::Unauthorized,
            Json(responses::Error {
                error: responses::ErrorStruct {
                    message: "unauthorized".to_owned(),
                    code: 50,
                },
            }),
        )),
    }
}

#[rocket::post("/<id>/unclaim")]
pub fn unclaim_item(
    id: i32,
    user: UserClaim,
) -> Result<status::Accepted<String>, (Status, Json<responses::Error>)> {
    match user.id.as_str() {
        "admin" => {
            unassign_item(id).map_err(|e| match e {
                DatabaseQueryError::NotUpdated => (
                    Status::Conflict,
                    Json(responses::Error {
                        error: responses::ErrorStruct {
                            message: e.to_string(),
                            code: 51,
                        },
                    }),
                ),
                _ => (
                    Status::InternalServerError,
                    Json(responses::Error {
                        error: responses::ErrorStruct {
                            message: e.to_string(),
                            code: 52,
                        },
                    }),
                ),
            })?;

            info!("Unclaimed item {id}");

            Ok(status::Accepted("Unclaimed".to_owned()))
        }
        _ => Err((
            Status::Unauthorized,
            Json(responses::Error {
                error: responses::ErrorStruct {
                    message: "unauthorized".to_owned(),
                    code: 53,
                },
            }),
        )),
    }
}

#[rocket::post("/<id>/link", data = "<link>")]
pub fn link_item(
    id: i32,
    user: UserClaim,
    link: Json<requests::Link>,
) -> Result<status::Accepted<String>, (Status, Json<responses::Error>)> {
    match user.id.as_str() {
        "admin" => {
            update_item_link(id, &link.link).map_err(|e| {
                (
                    Status::InternalServerError,
                    Json(responses::Error {
                        error: responses::ErrorStruct {
                            message: e.to_string(),
                            code: 54,
                        },
                    }),
                )
            })?;

            info!("Updated item {id} link to {}", link.link);

            Ok(status::Accepted("Link updated".to_owned()))
        }
        _ => Err((
            Status::Unauthorized,
            Json(responses::Error {
                error: responses::ErrorStruct {
                    message: "unauthorized".to_owned(),
                    code: 55,
                },
            }),
        )),
    }
}

#[rocket::options("/")]
pub fn options_items() -> rocket::response::status::Accepted<String> {
    rocket::response::status::Accepted("Accepted".to_owned())
}

#[rocket::options("/add")]
pub fn options_add_item() -> rocket::response::status::Accepted<String> {
    rocket::response::status::Accepted("Accepted".to_owned())
}

#[rocket::options("/<_id>/claim")]
pub fn options_claim_item(_id: i32) -> rocket::response::status::Accepted<String> {
    rocket::response::status::Accepted("Accepted".to_owned())
}

#[rocket::options("/<_id>/delete")]
pub fn options_remove_item(_id: i32) -> rocket::response::status::Accepted<String> {
    rocket::response::status::Accepted("Accepted".to_owned())
}

#[rocket::options("/<_id>/unclaim")]
pub fn options_unclaim_item(_id: i32) -> rocket::response::status::Accepted<String> {
    rocket::response::status::Accepted("Accepted".to_owned())
}

#[rocket::options("/<_id>/link")]
pub fn options_link_item(_id: i32) -> rocket::response::status::Accepted<String> {
    rocket::response::status::Accepted("Accepted".to_owned())
}
