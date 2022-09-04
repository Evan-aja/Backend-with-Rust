use sea_orm_migration::{prelude::*, sea_orm::{Set, ActiveModelTrait}};
use entity::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cars::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Cars::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Cars::Name).string().not_null())
                    .col(ColumnDef::new(Cars::Brand).string().not_null())
                    .to_owned(),
            )
            .await?;
            let db = manager.get_connection();

            cars::ActiveModel {
                name: Set("Innova".to_owned()),
                brand: Set("Toyota".to_owned()),
                ..Default::default()
            }
            .insert(db)
            .await?;

            Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cars::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Cars {
    Table,
    Id,
    Name,
    Brand,
}
