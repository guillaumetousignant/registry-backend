use rocket_jwt::jwt;

static SECRET_KEY: &str = "touche-poulet 2023-10-24 registre Ari & Guillaume";
#[jwt(SECRET_KEY)]
pub struct UserClaim {
    pub id: String,
    pub username: String,
}

#[jwt("secret", exp = 3600)]
pub struct UserClaimExp {
    pub id: String,
    pub username: String,
}

#[jwt("secret", leeway = 60)]
pub struct UserClaimLeeway {
    pub id: String,
    pub username: String,
}

// get token from cookie, key is `token`
#[jwt("secret", cookie = "token")]
pub struct UserClaimCookie {
    pub id: String,
    pub username: String,
}

// get token from request query, key is `token`
#[jwt("secret", query = "token")]
pub struct UserClaimQuery {
    pub id: String,
    pub username: String,
}
