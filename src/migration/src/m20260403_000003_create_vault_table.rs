use sea_orm_migration::{prelude::*};
use crate::m20260401_000002_create_device_table::Device;
use crate::m20260331_000001_create_user_table::User;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20260403_000002_create_vault_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Vault::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Vault::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Vault::Name).string().not_null())
                    .col(ColumnDef::new(Vault::CreatedByDeviceId).uuid().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk-created-by-device_id")
                                .from(Vault::Table, Vault::CreatedByDeviceId)
                                .to(Device::Table, Device::Id)
                        )
                    .col(ColumnDef::new(Vault::CreatedByUserId).uuid().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk-created-by-user_id")
                                .from(Vault::Table, Vault::CreatedByUserId)
                                .to(User::Table, User::Id)
                        )
                    .col(ColumnDef::new(Vault::CreatedAt).timestamp().not_null())
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Vault::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Vault {
    Table,
    Id,
    Name,
    CreatedByDeviceId,
    CreatedByUserId,
    CreatedAt,
}