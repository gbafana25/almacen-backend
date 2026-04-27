use crate::{api::{ErrorResponder, JWT, NetworkResponse, create_jwt}, entities::user};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};
use chrono::Utc;
use ::serde::{Deserialize, Serialize};
use entities::{prelude::*};
use sea_orm::{DatabaseConnection, prelude::Uuid, EntityTrait};
use rocket::{serde::json::Json, *};

use crate::entities;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SignupRequest<'r> {
    email: &'r str,
    password: &'r str,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest<'r> {
    id: Uuid,
    password: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserResponse {
    id: Uuid,
    email: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
    id: Uuid,
    email: String,
    jwt: String,
}

impl From<entities::user::Model> for UserResponse {
    fn from(model: entities::user::Model) -> Self {
        Self {
            id: model.id,
            email: model.email,
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

    let user: Option<user::Model> = User::find_by_id(request.id)
        .one(db)
        .await
        .unwrap();

    let user = user.unwrap();

    let saved_salt: PasswordHash = user.hashed_password.as_str().try_into().unwrap();
    match password_hasher.verify_password(request.password.to_owned().as_bytes(), &saved_salt) {
        Ok(_) => {
            //return Ok(Json(user.into()));
            match create_jwt(user.id) {
                Ok(token) => Ok(Json(LoginResponse { id: user.id, email: user.email, jwt: token })),
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
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = hash_password(&request.password.to_string(), &salt);
    let id = Uuid::new_v4();

    let new_user = entities::user::ActiveModel {
        id: sea_orm::ActiveValue::Set(id.to_owned()),
        email: sea_orm::ActiveValue::Set(request.email.to_owned()),
        created_at: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
        updated_at: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
        hashed_password: sea_orm::ActiveValue::Set(hashed_password),
        ..Default::default()
    };

    User::insert(new_user)
        .exec(db)
        .await?;
    Ok(Json(UserResponse { id: id, email: request.email.to_string() }))
}