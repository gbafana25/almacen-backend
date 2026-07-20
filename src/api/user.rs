use crate::{api::{ErrorResponder, JWT, NetworkResponse, create_jwt}, entities::user};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};
use chrono::Utc;
use ::serde::{Deserialize, Serialize};
use entities::{prelude::*};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, prelude::Uuid};
use rocket::{serde::json::Json, *};

use crate::entities;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SignupRequest<'r> {
    email: &'r str,
    password: &'r str,
    account_key: &'r str,
    acct_key_nonce: &'r str,
    salt: &'r str,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest<'r> {
    email: &'r str,
    password: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserResponse {
    id: Uuid,
    email: String,
    account_key: String,
    acct_key_nonce: String,
    salt: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
    id: Uuid,
    email: String,
    jwt: String,
    account_key: String,
    acct_key_nonce: String,
    salt: String,
}

impl From<entities::user::Model> for UserResponse {
    fn from(model: entities::user::Model) -> Self {
        Self {
            id: model.id,
            email: model.email,
            account_key: model.account_key,
            acct_key_nonce: model.account_key_nonce,
            salt: model.salt,
        }
    }
}

fn hash_password(key: &String, salt: &SaltString) -> String {
    Argon2::default().hash_password(key.as_bytes(), salt).unwrap().to_string()
}

// in the future will generate and store JWT token
#[post("/login", data = "<request>")]
pub async fn login(db: &State<DatabaseConnection>, request: Json<LoginRequest<'_>>) -> Result<Json<LoginResponse>, NetworkResponse> {
    let db = db as &DatabaseConnection;
    let password_hasher = Argon2::default();

    let user: Option<user::Model> = User::find()
        .filter(user::Column::Email.eq(request.email))
        .one(db)
        .await?;

    let user = user.unwrap();

    let saved_salt: PasswordHash = user.hashed_password.as_str().try_into().unwrap();
    match password_hasher.verify_password(request.password.to_owned().as_bytes(), &saved_salt) {
        Ok(_) => {
            //return Ok(Json(user.into()));
            match create_jwt(user.id) {
                Ok(token) => Ok(Json(LoginResponse { id: user.id, email: user.email, jwt: token, account_key: user.account_key, acct_key_nonce: user.account_key_nonce, salt: user.salt })),
                Err(err) => Err(NetworkResponse::BadRequest(err.to_string()))
            }
        },
        Err(err) => {
            return Err(NetworkResponse::BadRequest(err.to_string()));
        }
    }

}

#[get("/users")]
pub async fn users(db: &State<DatabaseConnection>) -> Json<Vec<UserResponse>> {
    let db = db as &DatabaseConnection;

    let user_objs: Vec<UserResponse> = User::find()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(Into::into)
        .collect();
    Json(user_objs)
}

#[get("/validate")]
pub async fn validate(key: Result<JWT, NetworkResponse>) -> Result<Json<String>, NetworkResponse> {
    match key {
        Ok(_) => Ok(Json(String::from("ok"))),
        Err(e) => {
            Err(e)
        },
    }
}

#[post("/signup", data = "<request>")]
pub async fn signup(db: &State<DatabaseConnection>, request: Json<SignupRequest<'_>>) -> Result<Json<UserResponse>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    match User::find().filter(user::Column::Email.eq(request.email)).one(db).await? {
        Some(_) => {
            return Err(ErrorResponder { message: "user already exists".to_string() })
        },
        None => {
            
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = hash_password(&request.password.to_string(), &salt);
    let id = Uuid::new_v4();

    let new_user = entities::user::ActiveModel {
        id: sea_orm::ActiveValue::Set(id.to_owned()),
        email: sea_orm::ActiveValue::Set(request.email.to_owned()),
        created_at: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
        updated_at: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
        hashed_password: sea_orm::ActiveValue::Set(hashed_password),
        account_key: sea_orm::ActiveValue::Set(request.account_key.to_owned()),
        account_key_nonce: sea_orm::ActiveValue::Set(request.acct_key_nonce.to_owned()),
        salt: sea_orm::ActiveValue::Set(request.salt.to_owned()),
        ..Default::default()
    };

    User::insert(new_user)
        .exec(db)
        .await?;
    Ok(Json(UserResponse { id: id, email: request.email.to_string(), account_key: request.account_key.to_string(), acct_key_nonce: request.acct_key_nonce.to_string(), salt: request.salt.to_string() }))
}