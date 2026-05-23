use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait, prelude::Uuid};
use ::serde::{Deserialize, Serialize};
use rocket::{serde::json::Json, *};
use crate::api::{JWT, NetworkResponse};
use crate::entities::{self, device};
use crate::entities::user::{self};
use crate::User;
use crate::Device;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DeviceResponse {
    id: Uuid,
    user_id: Uuid,
    name: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateDeviceRequest<'r> {
   name: &'r str,
}

impl From<entities::device::Model> for DeviceResponse {
    fn from(model: entities::device::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            user_id: model.user_id,
        }
    }
}

#[get("/devices")]
pub async fn get_devices(db: &State<DatabaseConnection>, key: Result<JWT, NetworkResponse>) -> Result<Json<Vec<DeviceResponse>>, NetworkResponse> {
    let _key = key?;
    let db = db as &DatabaseConnection;

    let user: Option<user::Model> = User::find_by_id(_key.claims.user_id).one(db).await.unwrap();
    let user: user::Model = user.unwrap();

    let devices = user.find_related(Device)
    .all(db)
    .await
    .unwrap()
    .into_iter()
    .map(Into::into)
    .collect();

    Ok(Json(devices))
}

#[post("/create-device", data = "<request>")]
pub async fn create_device(db: &State<DatabaseConnection>, request: Json<CreateDeviceRequest<'_>>, key: Result<JWT, NetworkResponse>) -> Result<Json<DeviceResponse>, NetworkResponse> {
    let _key = key?;
    let db = db as &DatabaseConnection;
    let id = Uuid::new_v4();
    
    let updated_device = device::ActiveModel {
        id: sea_orm::ActiveValue::Set(id.to_owned()),
        user_id: sea_orm::ActiveValue::Set(_key.claims.user_id),
        name: sea_orm::ActiveValue::Set(request.name.to_owned()),
        last_seen: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
        created_at: sea_orm::ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    match Device::insert(updated_device).exec(db).await {
        Ok(_) => Ok(Json(DeviceResponse { id, user_id: _key.claims.user_id, name: request.name.to_string() })),
        Err(err) => return Err(NetworkResponse::BadRequest(err.to_string())),
    }
    
}