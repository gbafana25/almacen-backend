use crate::api::ErrorResponder;
use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
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

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserResponse {
    id: Uuid,
    email: String,
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
        password_salt: sea_orm::ActiveValue::Set(salt.to_string()),
        hashed_password: sea_orm::ActiveValue::Set(hashed_password),
        ..Default::default()
    };

    User::insert(new_user)
        .exec(db)
        .await?;
    Ok(Json(UserResponse { id: id, email: request.email.to_string() }))
}