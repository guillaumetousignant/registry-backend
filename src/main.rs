use registry_backend::database::run_migrations;
use registry_backend::routes::authorize;
use registry_backend::routes::items;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use std::error::Error;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    run_migrations()?;

    let _rocket = rocket::build()
        .mount(
            "/api/v1/authorize",
            rocket::routes![
                authorize::authorize,
                authorize::authorize_admin,
                authorize::options_authorize,
                authorize::options_authorize_admin,
            ],
        )
        .mount(
            "/api/v1/items",
            rocket::routes![
                items::items,
                items::add_item,
                items::claim_item,
                items::remove_item,
                items::unclaim_item,
                items::link_item,
                items::options_items,
                items::options_add_item,
                items::options_claim_item,
                items::options_remove_item,
                items::options_unclaim_item,
                items::options_link_item,
            ],
        )
        .attach(CORS)
        .launch()
        .await?;

    Ok(())
}
