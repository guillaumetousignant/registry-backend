use rocket_jwt::jwt;

static SECRET_KEY: &'static str = include_str!(env!("REGISTRYSECRETKEY_FILE", "specify the path to the file containing the secret key with the REGISTRYSECRETKEY_FILE environment variable"));
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
