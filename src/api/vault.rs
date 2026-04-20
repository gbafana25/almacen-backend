use sea_orm::ModelTrait;
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, prelude::Uuid};
use ::serde::{Serialize, Deserialize};
use rocket::{serde::json::Json, *};

use crate::api::{JWT, NetworkResponse};
use crate::entities::prelude::VaultKey;
use crate::entities::{self, user};
use crate::Vault;
use crate::User;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct VaultResponse {
    id: Uuid,
    name: String,
    created_by_device_id: Uuid,
}

impl From<entities::vault::Model> for VaultResponse {
    fn from(model: entities::vault::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            created_by_device_id: model.created_by_device_id,
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateVaultRequest<'r> {
    name: &'r str,
    device_id: Uuid,
    user_id: Uuid,
    encrypted_vault_key: &'r str,
    nonce: &'r str,
}

#[get("/vaults/<user_id>")]
pub async fn get_all_vaults(db: &State<DatabaseConnection>, user_id: Uuid, key: Result<JWT, NetworkResponse>) -> Result<Json<Vec<VaultResponse>>, NetworkResponse> {
    let _key = key?;
    let db = db as &DatabaseConnection;

    let user: Option<user::Model> = User::find_by_id(user_id).one(db).await.unwrap();
    let user: user::Model = user.unwrap();

    let vaults = user.find_related(Vault)
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(Json(vaults))
}

#[post("/vaults", data = "<request>")]
pub async fn create_vault(db: &State<DatabaseConnection>, request: Json<CreateVaultRequest<'_>>, key: Result<JWT, NetworkResponse>) -> Result<Json<VaultResponse>, NetworkResponse> {
    let _key = key?;
    let db = db as &DatabaseConnection;
    let id = Uuid::new_v4();

    let new_vault = entities::vault::ActiveModel {
        id: sea_orm::ActiveValue::Set(id.to_owned()),
        name: sea_orm::ActiveValue::Set(request.name.to_owned()),
        created_by_device_id: sea_orm::ActiveValue::Set(request.device_id),
        created_by_user_id: sea_orm::ActiveValue::Set(request.user_id),
        created_at: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
    };

    Vault::insert(new_vault).exec(db).await.unwrap();
    
    let key_id = Uuid::new_v4();
    let new_vault_key = entities::vault_key::ActiveModel {
        id: sea_orm::ActiveValue::Set(key_id.to_owned()),
        vault_id: sea_orm::ActiveValue::Set(id.to_owned()),
        device_id: sea_orm::ActiveValue::Set(request.device_id),
        encrypted_vault_key: sea_orm::ActiveValue::Set(request.encrypted_vault_key.to_string()),
        nonce: sea_orm::ActiveValue::Set(request.nonce.to_string()),
        created_at: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
    };

    VaultKey::insert(new_vault_key).exec(db).await.unwrap();

    Ok(Json(VaultResponse { id, name: request.name.to_string(), created_by_device_id: request.device_id }))
}