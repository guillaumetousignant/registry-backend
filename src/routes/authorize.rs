use super::responses;
use rocket::serde::json::Json;

#[rocket::get("/", format = "json")]
pub async fn authorize() -> Result<Json<responses::Authorize>, Json<responses::Error>> {
    Ok(Json(responses::Authorize { authorized: true }))
}
