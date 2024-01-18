use crate::utilities::{BasicAuth, BasicAuthError};
use dotenvy::dotenv;
use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
};
use std::env;

pub struct Admin {
    pub username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = BasicAuthError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let basic = BasicAuth::from_request(req).await;

        dotenv().ok();
        let key = env::var("ADMIN_PASSWORD");
        let key = match key {
            Ok(key) => key,
            Err(e) => {
                return request::Outcome::Error((
                    Status::Unauthorized,
                    BasicAuthError::KeyNotSet(e),
                ))
            }
        };

        match basic {
            request::Outcome::Success(basic) => match basic.password == key {
                true => request::Outcome::Success(Admin {
                    username: basic.username,
                }),
                false => {
                    request::Outcome::Error((Status::Unauthorized, BasicAuthError::Unauthorized))
                }
            },
            request::Outcome::Error(e) => request::Outcome::Error(e),
            request::Outcome::Forward(e) => request::Outcome::Forward(e),
        }
    }
}
