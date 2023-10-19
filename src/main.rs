use registry_backend::routes::authorize;
use std::error::Error;

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let _rocket = rocket::build()
        .mount("/", rocket::routes![index])
        .mount("/api/v1/authorize", rocket::routes![authorize::authorize])
        .launch()
        .await?;

    Ok(())
}
