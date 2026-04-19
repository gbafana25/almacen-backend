use chrono::Utc;
use rocket::post;
use rocket::{State, get, serde::json::Json};
use sea_orm::{DatabaseConnection, ModelTrait, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::ErrorResponder;
use crate::entities::{self, vault};
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

#[get("/vault-items/<vault_id>")]
pub async fn get_vault_items_by_vault(db: &State<DatabaseConnection>, vault_id: Uuid) -> Json<Vec<VaultItemResponse>> {
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

    return Json(vault_items);
}

#[post("/vault-items", data = "<request>")]
pub async fn create_vault_item(db: &State<DatabaseConnection>, request: Json<CreateVaultItemRequest<'_>>) -> Result<Json<VaultItemResponse>, ErrorResponder> {
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
        .await?;

    Ok(Json(VaultItemResponse { id, name: request.name.to_owned(), vault_id: request.vault_id, ciphertext: "".to_owned(), nonce: "".to_owned() }))
}
