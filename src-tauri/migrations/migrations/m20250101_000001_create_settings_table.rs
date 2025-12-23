//! Migration: Create settings table

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create settings table
        manager
            .create_table(
                Table::create()
                    .table(Settings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Settings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Settings::Key)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Settings::Value)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Settings::Category)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Settings::Description)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Settings::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Settings::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on key
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_settings_key")
                    .table(Settings::Table)
                    .col(Settings::Key)
                    .to_owned(),
            )
            .await?;

        // Create index on category
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_settings_category")
                    .table(Settings::Table)
                    .col(Settings::Category)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes
        manager
            .drop_index(Index::drop().name("idx_settings_category").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_settings_key").to_owned())
            .await?;

        // Drop table
        manager
            .drop_table(Table::drop().table(Settings::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Define the table and column identifiers
#[derive(DeriveIden)]
enum Settings {
    Table,
    Id,
    Key,
    Value,
    Category,
    Description,
    CreatedAt,
    UpdatedAt,
}
