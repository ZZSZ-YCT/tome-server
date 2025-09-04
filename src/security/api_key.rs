use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{Error, FromRequest, HttpRequest};
use std::future::{Ready, ready};

pub struct AuthenticatedUser(pub String);

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(settings) = req.app_data::<Data<crate::config::Settings>>() {
            if let Some(auth_header) = req.headers().get("Authorization")
                && let Ok(auth_str) = auth_header.to_str()
                && auth_str.starts_with("Bearer ")
            {
                let token = &auth_str[7..];

                if token == settings.api_key || settings.api_key.is_empty() {
                    return ready(Ok(AuthenticatedUser(token.to_string())));
                }
            }

            return ready(Err(actix_web::error::ErrorUnauthorized(
                "Invalid or missing token",
            )));
        }

        ready(Err(actix_web::error::ErrorInternalServerError("Unknown")))
    }
}
