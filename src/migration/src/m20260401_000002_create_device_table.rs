use sea_orm_migration::{prelude::*};
use crate::m20260331_000001_create_user_table::User;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20260401_000002_create_device_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Device::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Device::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Device::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-device-user_id")
                            .from(Device::Table, Device::UserId)
                            .to(User::Table, User::Id)
                    )
                    .col(ColumnDef::new(Device::Name).string().not_null())
                    .col(ColumnDef::new(Device::LastSeen).timestamp().not_null())
                    .col(ColumnDef::new(Device::CreatedAt).timestamp().not_null())
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Device::Table).to_owned())
            .await
    }


}

#[derive(DeriveIden)]
pub enum Device {
    Table,
    Id,
    UserId,
    Name,
    LastSeen,
    CreatedAt,
}