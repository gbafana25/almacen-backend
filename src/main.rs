// src/main.rs
mod database;
mod entities;
mod api;
use entities::{prelude::*};

use rocket::*;
use database::set_up_db;
use crate::api::{device::{create_device, get_devices}, user::{signup, users}, vault::{create_vault, get_vaults_by_device}, vault_item::{create_vault_item, get_vault_items_by_vault}};


#[launch] // The "main" function of the program
async fn rocket() -> _ {
    let db = match set_up_db().await {
       Ok(db) => db,
       Err(err) => panic!("{}", err),
    };

    rocket::build()
        .manage(db)
        .mount("/", routes![users, signup, get_devices, create_device, get_vaults_by_device, create_vault, get_vault_items_by_vault, create_vault_item])
}

