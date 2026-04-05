use sea_orm_migration::{prelude::*, schema::*};
use crate::m20260403_000003_create_vault_table::Vault;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20260405_000005_create_vault_item_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VaultItem::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(VaultItem::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(VaultItem::VaultId).uuid().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk-vault-item-vault_id")
                                .from(VaultItem::Table, VaultItem::VaultId)
                                .to(Vault::Table, Vault::Id)
                        )
                    .col(ColumnDef::new(VaultItem::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(VaultItem::UpdatedAt).timestamp().not_null())
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VaultItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum VaultItem {
    Table,
    Id,
    VaultId,
    Ciphertext,
    Nonce,
    CreatedAt,
    UpdatedAt,
}