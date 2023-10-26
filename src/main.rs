use registry_backend::database::run_migrations;
use registry_backend::routes::authorize;
use registry_backend::routes::items;
use std::error::Error;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    run_migrations()?;

    let _rocket = rocket::build()
        .mount(
            "/api/v1/authorize",
            rocket::routes![
                authorize::authorize,
                authorize::authorize_admin,
                authorize::get_user_id_from_jwt,
                authorize::get_username_from_jwt,
            ],
        )
        .mount(
            "/api/v1/items",
            rocket::routes![items::items, items::add_item, items::claim_item],
        )
        .launch()
        .await?;

    Ok(())
}
