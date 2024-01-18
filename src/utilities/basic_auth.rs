use base64::Engine;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use std::env::VarError;
use thiserror::Error;

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

#[derive(Error, Debug)]
pub enum BasicAuthError {
    #[error("authorization header should have two words, found {0}")]
    IncorrectHeaderFormat(usize),
    #[error("incorrect authorization type, expected \"Basic\", found \"{0}\"")]
    IncorrectAuthType(String),
    #[error("base64 decode error")]
    Base64Decode(#[from] base64::DecodeError),
    #[error("utf-8 decode error")]
    StringDecode(#[from] std::string::FromUtf8Error),
    #[error("credentials should have two words, found {0}")]
    IncorrectCredentialsFormat(usize),
    #[error("missing Authorization header")]
    MissingHeader,
    #[error("unauthorized")]
    Unauthorized,
    #[error("key not set")]
    KeyNotSet(#[from] VarError),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = BasicAuthError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth_header = req.headers().get_one("Authorization");
        match auth_header {
            Some(auth_header) => {
                let words = auth_header.split_whitespace().collect::<Vec<_>>();
                if words.len() != 2 {
                    return request::Outcome::Error((
                        Status::Unauthorized,
                        BasicAuthError::IncorrectHeaderFormat(words.len()),
                    ));
                }

                let (auth_type, credentials) = (words[0], words[1]);
                if auth_type != "Basic" {
                    return request::Outcome::Error((
                        Status::Unauthorized,
                        BasicAuthError::IncorrectAuthType(auth_type.to_owned()),
                    ));
                }

                let credentials = base64::engine::general_purpose::STANDARD.decode(credentials);
                let credentials = match credentials {
                    Ok(credentials) => credentials,
                    Err(e) => {
                        return request::Outcome::Error((
                            Status::BadRequest,
                            BasicAuthError::Base64Decode(e),
                        ));
                    }
                };

                let credentials = String::from_utf8(credentials);
                let credentials = match credentials {
                    Ok(credentials) => credentials,
                    Err(e) => {
                        return request::Outcome::Error((
                            Status::BadRequest,
                            BasicAuthError::StringDecode(e),
                        ));
                    }
                };

                let words = credentials.split(":").collect::<Vec<_>>();
                if words.len() != 2 {
                    return request::Outcome::Error((
                        Status::BadRequest,
                        BasicAuthError::IncorrectCredentialsFormat(words.len()),
                    ));
                }

                let (username, password) = (words[0].to_owned(), words[1].to_owned());

                request::Outcome::Success(BasicAuth { username, password })
            }
            None => request::Outcome::Error((Status::Unauthorized, BasicAuthError::MissingHeader)),
        }
    }
}
