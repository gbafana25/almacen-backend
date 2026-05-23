use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ModelTrait};
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, prelude::Uuid};
use ::serde::{Serialize, Deserialize};
use rocket::{serde::json::Json, *};

use crate::api::{JWT, NetworkResponse};
use crate::entities::{self, user, vault};
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
pub struct EditVaultRequest<'r> {
    name: &'r str,
    device_id: Uuid,
}

#[put("/vaults/<vault_id>", data = "<request>")]
pub async fn update_vault(db: &State<DatabaseConnection>, vault_id: Uuid, request: Json<EditVaultRequest<'_>>, key: Result<JWT, NetworkResponse>) -> Result<Json<VaultResponse>, NetworkResponse> {
    let _key = key?;
    let db = db as &DatabaseConnection;

    let vault: Option<vault::Model> = Vault::find_by_id(vault_id).one(db).await.unwrap();
    let mut vault: vault::ActiveModel = vault.unwrap().into();

    vault.created_by_device_id = Set(request.device_id.to_owned());
    vault.name = Set(request.name.to_owned());

    let vault: vault::Model = vault.update(db).await?;

    Ok(Json(vault.into()))


}

#[get("/vaults")]
pub async fn get_all_vaults(db: &State<DatabaseConnection>, key: Result<JWT, NetworkResponse>) -> Result<Json<Vec<VaultResponse>>, NetworkResponse> {
    let _key = key?;
    let db = db as &DatabaseConnection;

    let user: Option<user::Model> = User::find_by_id(_key.claims.user_id).one(db).await.unwrap();
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
pub async fn create_vault(db: &State<DatabaseConnection>, request: Json<EditVaultRequest<'_>>, key: Result<JWT, NetworkResponse>) -> Result<Json<VaultResponse>, NetworkResponse> {
    let _key = key?;
    let db = db as &DatabaseConnection;
    let id = Uuid::new_v4();

    let new_vault = entities::vault::ActiveModel {
        id: sea_orm::ActiveValue::Set(id.to_owned()),
        name: sea_orm::ActiveValue::Set(request.name.to_owned()),
        created_by_device_id: sea_orm::ActiveValue::Set(request.device_id),
        created_by_user_id: sea_orm::ActiveValue::Set(_key.claims.user_id),
        created_at: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
    };

    Vault::insert(new_vault).exec(db).await.unwrap();

    Ok(Json(VaultResponse { id, name: request.name.to_string(), created_by_device_id: request.device_id }))
}