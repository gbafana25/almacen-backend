// src/main.rs
use sea_orm_migration::prelude::*;
mod database;
mod entities;
use chrono::{self, Utc};

use entities::{prelude::*, *};

use rocket::{serde::json::Json, *};
use database::set_up_db;
use sea_orm::{DatabaseConnection, EntityTrait};

#[get("/")]
async fn index() -> &'static str {
    "Hello, bakeries!"
}

#[get("/users")]
async fn users(db: &State<DatabaseConnection>) -> Json<Vec<String>> {
    let db = db as &DatabaseConnection;

    let user_objs = User::find()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|b| b.email)
        .collect::<Vec<String>>();
    Json(user_objs)
}

#[get("/create-user?<email>")]
async fn create_user(db: &State<DatabaseConnection>, email: &str) -> Result<(), ErrorResponder> {
    let db = db as &DatabaseConnection;

    let new_user = user::ActiveModel {
        email: sea_orm::ActiveValue::Set(email.to_owned()),
        created_at: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
        updated_at: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
        password_salt: sea_orm::ActiveValue::Set("temp_pass_salt".as_bytes().to_vec()),
        ..Default::default()
    };

    User::insert(new_user)
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
        .mount("/", routes![index, users, create_user])
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