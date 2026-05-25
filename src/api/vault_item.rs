use chrono::Utc;
use rocket::{post, put};
use rocket::{State, get, serde::json::Json};
use sea_orm::{ActiveModelTrait, Set};
use sea_orm::{DatabaseConnection, ModelTrait, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::{JWT, NetworkResponse};
use crate::entities::{self, vault, vault_item};
use crate::Vault;
use crate::VaultItem;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct VaultItemResponse {
    id: Uuid,
    name: String,
    vault_id: Uuid,
    ciphertext: String,
    nonce: String,
}

impl From<entities::vault_item::Model> for VaultItemResponse {
    fn from(model: entities::vault_item::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            vault_id: model.vault_id,
            ciphertext: model.ciphertext,
            nonce: model.nonce,
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateVaultItemRequest<'r> {
    name: &'r str,
    vault_id: Uuid,
    encrypted_item: &'r str,
    nonce: &'r str,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EditVaultItemRequest<'r> {
    name: &'r str,
    encrypted_item: &'r str,
    nonce: &'r str,
}

#[get("/vault-items/item/<vault_item_id>")]
pub async fn get_vault_item(db: &State<DatabaseConnection>, vault_item_id: Uuid, key: Result<JWT, NetworkResponse>) -> Result<Json<VaultItemResponse>, NetworkResponse> {
    let _key = key?;
    let db = db as &DatabaseConnection;

    let vault_item: Option<vault_item::Model> = VaultItem::find_by_id(vault_item_id).one(db).await?;
    let vault_item = vault_item.unwrap();

    Ok(Json(vault_item.into()))
}

#[put("/vault-items/<vault_item_id>", data = "<request>")]
pub async fn update_vault_item(db: &State<DatabaseConnection>, vault_item_id: Uuid, request: Json<EditVaultItemRequest<'_>>, key: Result<JWT, NetworkResponse>) -> Result<Json<VaultItemResponse>, NetworkResponse> {
    let _key = key?;
    let db = db as &DatabaseConnection;

    let vault_item: Option<vault_item::Model> = VaultItem::find_by_id(vault_item_id).one(db).await?;
    let mut vault_item: vault_item::ActiveModel = vault_item.unwrap().into();

    vault_item.name = Set(request.name.to_owned());
    vault_item.ciphertext = Set(request.encrypted_item.to_owned());
    vault_item.nonce = Set(request.nonce.to_owned());

    let vault_item: vault_item::Model = vault_item.update(db).await?;
    Ok(Json(vault_item.into()))

}

#[get("/vault-items/vault/<vault_id>")]
pub async fn get_vault_items_by_vault(db: &State<DatabaseConnection>, vault_id: Uuid, key: Result<JWT, NetworkResponse>) -> Result<Json<Vec<VaultItemResponse>>, NetworkResponse> {
    let _key = key?;
    let db = db as &DatabaseConnection;

    let vault: Option<vault::Model> = Vault::find_by_id(vault_id).one(db).await.unwrap();

    let vault = vault.unwrap();

    let vault_items: Vec<VaultItemResponse> = vault.find_related(VaultItem)
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(Json(vault_items))
}

#[post("/vault-items", data = "<request>")]
pub async fn create_vault_item(db: &State<DatabaseConnection>, request: Json<CreateVaultItemRequest<'_>>, key: Result<JWT, NetworkResponse>) -> Result<Json<VaultItemResponse>, NetworkResponse> {
    let _key = key?;
    let db = db as &DatabaseConnection;
    let id = Uuid::new_v4();

    let new_vault_item = entities::vault_item::ActiveModel {
        id: sea_orm::ActiveValue::Set(id),
        name: sea_orm::ActiveValue::Set(request.name.to_owned()),
        vault_id: sea_orm::ActiveValue::Set(request.vault_id),
        ciphertext: sea_orm::ActiveValue::Set(request.encrypted_item.to_owned()),
        nonce: sea_orm::ActiveValue::Set(request.nonce.to_owned()),
        created_at: sea_orm::ActiveValue::Set(Utc::now().naive_local()),
        updated_at: sea_orm::ActiveValue::Set(Utc::now().naive_local()),
    };

    VaultItem::insert(new_vault_item)
        .exec(db)
        .await.unwrap();

    Ok(Json(VaultItemResponse { id, name: request.name.to_owned(), vault_id: request.vault_id, ciphertext: "".to_owned(), nonce: "".to_owned() }))
}
