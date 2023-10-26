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

#[rocket::get("/user_id")]
pub fn get_user_id_from_jwt(user: UserClaim) -> String {
    format!("user id is {}", user.id)
}

#[rocket::get("/username")]
pub fn get_username_from_jwt(user: UserClaim) -> String {
    format!("username is {}", user.username)
}
