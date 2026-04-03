// src/main.rs
use sea_orm_migration::prelude::*;
mod database;
mod entities;
use chrono::{self, Utc};

use entities::{prelude::*, *};

use rocket::{serde::json::Json, *};
use database::set_up_db;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait, prelude::Uuid};
use ::serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct SignupRequest<'r> {
    email: &'r str,
    password: &'r str,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct CreateDeviceRequest<'r> {
   name: &'r str,
   identity_public_key: &'r str,
   user_id: Uuid,
}

#[derive(Serialize)]
struct UserResponse {
    id: Uuid,
    email: String,
}

#[get("/users")]
async fn users(db: &State<DatabaseConnection>) -> Json<Vec<UserResponse>> {
    let db = db as &DatabaseConnection;

    let user_objs = User::find()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|b| UserResponse { email: b.email.clone(), id: b.id })
        .collect::<Vec<UserResponse>>();
    Json(user_objs)
}

#[post("/signup", data = "<request>")]
async fn signup(db: &State<DatabaseConnection>, request: Json<SignupRequest<'_>>) -> Result<(), ErrorResponder> {
    let db = db as &DatabaseConnection;

    let new_user = user::ActiveModel {
        email: sea_orm::ActiveValue::Set(request.email.to_owned()),
        created_at: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
        updated_at: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
        password_salt: sea_orm::ActiveValue::Set(request.password.as_bytes().to_vec()),
        ..Default::default()
    };

    User::insert(new_user)
        .exec(db)
        .await?;
    Ok(())
}

#[get("/devices/<user_id>")]
async fn get_devices(db: &State<DatabaseConnection>, user_id: Uuid) -> Json<Vec<String>> {
    let db = db as &DatabaseConnection;

    let user: Option<user::Model> = User::find_by_id(user_id).one(db).await.unwrap();
    let user: user::Model = user.unwrap();

    let devices = user.find_related(Device)
    .all(db)
    .await
    .unwrap()
    .into_iter()
    .map(|d| d.name)
    .collect::<Vec<String>>();

    Json(devices)
}

#[post("/create-device", data = "<request>")]
async fn create_device(db: &State<DatabaseConnection>, request: Json<CreateDeviceRequest<'_>>) -> Result<(), ErrorResponder> {
    let db = db as &DatabaseConnection;
    
    let updated_device = device::ActiveModel {
        user_id: sea_orm::ActiveValue::Set(request.user_id),
        name: sea_orm::ActiveValue::Set(request.name.to_owned()),
        identity_public_key: sea_orm::ActiveValue::Set(request.identity_public_key.as_bytes().to_vec()),
        last_seen: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
        created_at: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    Device::insert(updated_device)
        .exec(db)
        .await?;
    Ok(())
}

#[launch] // The "main" function of the program
async fn rocket() -> _ {
    let db = match set_up_db().await {
       Ok(db) => db,
       Err(err) => panic!("{}", err),
    };

    rocket::build()
        .manage(db)
        .mount("/", routes![users, signup, get_devices, create_device])
}

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct ErrorResponder {
    message: String,
}

impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> ErrorResponder {
        ErrorResponder {
            message: err.to_string(),
        }
    }
}