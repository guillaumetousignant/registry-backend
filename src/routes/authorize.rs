use super::authorization::{Admin, User, UserClaim};

#[rocket::get("/")]
pub fn authorize(user: User) -> String {
    let user_claim = UserClaim {
        id: "user".to_owned(),
        username: user.username,
    };
    let token = UserClaim::sign(user_claim);
    token
}

#[rocket::get("/admin")]
pub fn authorize_admin(admin: Admin) -> String {
    let user_claim = UserClaim {
        id: "admin".to_owned(),
        username: admin.username,
    };
    let token = UserClaim::sign(user_claim);
    token
}

#[rocket::options("/")]
pub fn options_authorize() -> rocket::response::status::Accepted<String> {
    rocket::response::status::Accepted(Some("Accepted".to_owned()))
}

#[rocket::options("/admin")]
pub fn options_authorize_admin() -> rocket::response::status::Accepted<String> {
    rocket::response::status::Accepted(Some("Accepted".to_owned()))
}
