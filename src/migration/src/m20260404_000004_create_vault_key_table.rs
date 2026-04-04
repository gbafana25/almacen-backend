use crate::m20260401_000002_create_device_table::Device;
use crate::m20260403_000003_create_vault_table::Vault;
use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20260403_000004_create_vault_key_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VaultKey::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(VaultKey::Id).uuid().not_null().primary_key().default(Expr::cust("gen_random_uuid()")))
                    .col(ColumnDef::new(VaultKey::VaultId).uuid().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk-vault-key-vault_id")
                                .from(VaultKey::Table, VaultKey::VaultId)
                                .to(Vault::Table, Vault::Id)
                        )
                    .col(ColumnDef::new(VaultKey::DeviceId).uuid().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk-vault-key-device_id")
                                .from(VaultKey::Table, VaultKey::DeviceId)
                                .to(Device::Table, Device::Id)
                        )
                    .col(ColumnDef::new(VaultKey::EncryptedVaultKey).binary().not_null())
                    .col(ColumnDef::new(VaultKey::Nonce).binary().not_null())
                    .col(ColumnDef::new(VaultKey::CreatedAt).timestamp().not_null())
                    .to_owned()
            )
            .await?;

            manager
                .create_index(
                    Index::create()
                        .name("idx-vault-device-unique")
                        .table(VaultKey::Table)
                        .col(VaultKey::VaultId)
                        .col(VaultKey::DeviceId)
                        .unique()
                        .to_owned(),
                )
                .await?;

            Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VaultKey::Table).to_owned())
            .await
    }

}

#[derive(DeriveIden)]
enum VaultKey {
    Table,
    Id,
    VaultId,
    DeviceId,
    EncryptedVaultKey,
    Nonce,
    CreatedAt,
}