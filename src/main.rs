// src/main.rs
mod database;
mod entities;
mod api;
use entities::{prelude::*};

use rocket::{http::Method, *};
use database::set_up_db;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use crate::api::{device::{create_device, get_devices}, user::{login, signup, users, validate}, vault::{create_vault, get_all_vaults, get_vault, update_vault}, vault_item::{create_vault_item, get_vault_item, get_vault_items_by_vault, update_vault_item}, vault_key::get_vault_key};
extern crate dotenv;
use dotenv::dotenv;
use std::{env, net::Ipv4Addr, str::FromStr};

#[launch] // The "main" function of the program
async fn rocket() -> _ {
    dotenv().ok();

    let config = Config {
        port: env::var("APP_PORT").unwrap_or(String::from("8000")).parse().unwrap_or(8000),
        address: Ipv4Addr::from_str(&env::var("APP_ADDR").unwrap_or("0.0.0.0".to_string())).unwrap_or(Ipv4Addr::new(0, 0, 0, 0).into()).into(),
        ..Default::default()
    };

    let db = match set_up_db().await {
       Ok(db) => db,
       Err(err) => panic!("{}", err),
    };

    rocket::custom(config)
        .manage(db)
        .mount("/", routes![users, signup, get_devices, create_device, get_all_vaults, get_vault, create_vault, get_vault_items_by_vault, update_vault, create_vault_item, get_vault_item, update_vault_item, login, get_vault_key, validate])
        .attach(cors_setup())
}

fn cors_setup() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:7000",
        "http://127.0.0.1:7000",
        "http://localhost:8081",
        "http://127.0.0.1:8081",
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap()
}
