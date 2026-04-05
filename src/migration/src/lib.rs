use sea_orm_migration::prelude::*;

mod m20260331_000001_create_user_table;
mod m20260401_000002_create_device_table;
mod m20260403_000003_create_vault_table;
mod m20260404_000004_create_vault_key_table;
mod m20260405_000005_create_vault_item_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260331_000001_create_user_table::Migration),
            Box::new(m20260401_000002_create_device_table::Migration),
            Box::new(m20260403_000003_create_vault_table::Migration),
            Box::new(m20260404_000004_create_vault_key_table::Migration),
            Box::new(m20260405_000005_create_vault_item_table::Migration),
        ]
    }
}
