use crate::{api::ErrorResponder, entities::user};
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
    password_hash: &'r str,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest<'r> {
    id: Uuid,
    password_hash: &'r str,
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

// in the future will generate and store JWT token
#[post("/login", data = "<request>")]
pub async fn login(db: &State<DatabaseConnection>, request: Json<LoginRequest<'_>>) -> Result<Json<UserResponse>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let user: Option<user::Model> = User::find_by_id(request.id)
        .one(db)
        .await
        .unwrap();

    let user = user.unwrap();

    if user.hashed_password == request.password_hash {
        Ok(Json(user.into()))
    } else {
        return Err(ErrorResponder { message: "invalid login info".to_string() });
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

#[post("/signup", data = "<request>")]
pub async fn signup(db: &State<DatabaseConnection>, request: Json<SignupRequest<'_>>) -> Result<Json<UserResponse>, ErrorResponder> {
    let db = db as &DatabaseConnection;
    let id = Uuid::new_v4();

    let new_user = entities::user::ActiveModel {
        id: sea_orm::ActiveValue::Set(id.to_owned()),
        email: sea_orm::ActiveValue::Set(request.email.to_owned()),
        created_at: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
        updated_at: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
        hashed_password: sea_orm::ActiveValue::Set(request.password_hash.to_owned()),
        ..Default::default()
    };

    User::insert(new_user)
        .exec(db)
        .await?;
    Ok(Json(UserResponse { id: id, email: request.email.to_string() }))
}