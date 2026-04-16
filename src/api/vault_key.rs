use crate::entities::{self, prelude::VaultKey, vault};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait, prelude::Uuid};
use ::serde::{Serialize};
use rocket::{serde::json::Json, *};
use crate::Vault;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct VaultKeyResponse {
    id: Uuid,
    vault_id: Uuid,
    encrypted_vault_key: String,
    nonce: String,
}

impl From<entities::vault_key::Model>for VaultKeyResponse {
    fn from(model: entities::vault_key::Model) -> Self {
        Self {
            id: model.id,
            vault_id: model.vault_id,
            encrypted_vault_key: model.encrypted_vault_key,
            nonce: model.nonce,
        }
    }
}

#[get("/vault-key/<vault_id>")]
pub async fn get_vault_key(db: &State<DatabaseConnection>, vault_id: Uuid) -> Option<Json<VaultKeyResponse>> {
    let db = db as &DatabaseConnection;

    let vault: Option<vault::Model> = Vault::find_by_id(vault_id).one(db).await.unwrap();
    let vault = vault.unwrap();

    let vault_key = vault.find_related(VaultKey)
        .one(db)
        .await
        .unwrap()?;

    Some(Json(vault_key.into()))
}