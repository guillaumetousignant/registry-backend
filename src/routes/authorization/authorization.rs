use rocket_jwt::jwt;

static SECRET_KEY: &'static str = include_str!(env!(
    "REGISTRYSECRETKEY_FILE",
    "specify the path to the file containing the secret key"
));
#[jwt(SECRET_KEY, exp = 3600, leeway = 60)]
pub struct UserClaim {
    pub id: String,
    pub username: String,
}
