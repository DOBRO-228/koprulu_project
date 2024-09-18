use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Sports {
    Table,
    Id,
    Name,
    Description,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sports::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sports::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sports::Name).string().not_null())
                    .col(ColumnDef::new(Sports::Description).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sports::Table).to_owned())
            .await
    }
}
