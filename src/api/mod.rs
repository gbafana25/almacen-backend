pub mod user;
pub mod device;
pub mod vault;
pub mod vault_item;
pub mod vault_key;

use chrono::Utc;
use rocket::{Responder, http::Status, request::{FromRequest, Outcome, Request}};
use sea_orm::DbErr;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::{Error, ErrorKind}};
use rocket::serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
pub struct ErrorResponder {
    message: String,
}

impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> ErrorResponder {
        ErrorResponder {
            message: err.to_string(),
        }
    }
}

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
}

impl From<sea_orm::DbErr> for NetworkResponse {
    fn from(value: sea_orm::DbErr) -> Self {
        Self::BadRequest(format!("Database error: {}", value))
    }
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub user_id: Uuid,
    exp: usize
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => {
                let response = Response { body: ResponseBody::Message(String::from("Error validating JWT token"))};
                Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())))
            },
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let response = Response { body: ResponseBody::Message(format!("Error: Expired token"))};
                        Outcome::Error((Status::NotFound, NetworkResponse::NotFound(serde_json::to_string(&response).unwrap())))
                    },
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        let response = Response { body: ResponseBody::Message(format!("Error: invalid token"))};
                        Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())))
                    },
                    _ => {
                        let response = Response { body: ResponseBody::Message(format!("Error validating JWT token - {}", err))};
                        Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())))
                    }
                }
            }
        }
    }
}

pub fn create_jwt(user_id: Uuid) -> Result<String, Error> {
    let secret = "Temporary secret";
    let exp = Utc::now().checked_add_signed(chrono::Duration::seconds(3600)).unwrap().timestamp();

    let claims = Claims {
        user_id: user_id,
        exp: exp as usize,
    };

    let header = Header::new(jsonwebtoken::Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
    
}

pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let token = token.trim_start_matches("Bearer").trim();
    let secret = "Temporary secret";

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS512)
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned())
    }

}