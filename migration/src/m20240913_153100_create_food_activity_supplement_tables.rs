use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240913_153100_create_food_activity_supplement_tables" // Make sure this matches with the file name
    }
}

#[derive(Iden)]
enum Activities {
    Table,
    Id,
    Name,
    Description,
}

#[derive(Iden)]
enum Foods {
    Table,
    Id,
    Name,
    Description,
}

#[derive(Iden)]
enum Supplements {
    Table,
    Id,
    Name,
    Description,
    DosageAndAdministration,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create activities table
        manager
            .create_table(
                Table::create()
                    .table(Activities::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Activities::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Activities::Name).string().not_null())
                    .col(ColumnDef::new(Activities::Description).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Create foods table
        manager
            .create_table(
                Table::create()
                    .table(Foods::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Foods::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Foods::Name).string().not_null())
                    .col(ColumnDef::new(Foods::Description).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Supplements::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Supplements::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Supplements::Name).string().not_null())
                    .col(ColumnDef::new(Supplements::Description).string().not_null())
                    .col(
                        ColumnDef::new(Supplements::DosageAndAdministration)
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order of creation
        manager
            .drop_table(Table::drop().table(Activities::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Foods::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Supplements::Table).to_owned())
            .await
    }
}
