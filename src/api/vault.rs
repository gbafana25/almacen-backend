use sea_orm::{DatabaseConnection, ModelTrait, EntityTrait, prelude::Uuid};
use ::serde::{Serialize, Deserialize};
use rocket::{serde::json::Json, *};

use crate::api::ErrorResponder;
use crate::entities::{self, device};
use crate::Device;
use crate::Vault;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct VaultResponse {
    id: Uuid,
    name: String
}

impl From<entities::vault::Model> for VaultResponse {
    fn from(model: entities::vault::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateVaultRequest<'r> {
    name: &'r str,
    device_id: Uuid,
}

#[get("/vaults/<device_id>")]
pub async fn get_vaults_by_device(db: &State<DatabaseConnection>, device_id: Uuid) -> Json<Vec<VaultResponse>> {
    let db = db as &DatabaseConnection;

    let device: Option<device::Model> = Device::find_by_id(device_id).one(db).await.unwrap();

    let device: device::Model = device.unwrap();

    let vaults: Vec<VaultResponse> = device.find_related(Vault)
    .all(db)
    .await
    .unwrap()
    .into_iter()
    .map(Into::into)
    .collect();

    return Json(vaults);
}

#[post("/vaults", data = "<request>")]
pub async fn create_vault(db: &State<DatabaseConnection>, request: Json<CreateVaultRequest<'_>>) -> Result<(), ErrorResponder> {
    let db = db as &DatabaseConnection;

    let new_vault = entities::vault::ActiveModel {
        name: sea_orm::ActiveValue::Set(request.name.to_owned()),
        created_by_device_id: sea_orm::ActiveValue::Set(request.device_id),
        ..Default::default()
    };

    Vault::insert(new_vault)
        .exec(db)
        .await?;

    Ok(())
}