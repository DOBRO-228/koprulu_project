use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240814_154410_create_init_tables" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create triggers table
        manager
            .create_table(
                Table::create()
                    .table(Trigger::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Trigger::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Trigger::Description).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Create descriptions table
        manager
            .create_table(
                Table::create()
                    .table(Description::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Description::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Description::Description).string().not_null())
                    .col(ColumnDef::new(Description::MetaDescription).string().null())
                    .col(ColumnDef::new(Description::InExcess).string().null())
                    .col(ColumnDef::new(Description::InNorm).string().null())
                    .col(ColumnDef::new(Description::InDeficiency).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Hormone::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Hormone::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Hormone::Name).string().not_null())
                    .col(ColumnDef::new(Hormone::HormoneType).string().not_null())
                    .col(ColumnDef::new(Hormone::DescriptionId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-description_id")
                            .from(Hormone::Table, Hormone::DescriptionId)
                            .to(Description::Table, Description::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create hormone_triggers table
        manager
            .create_table(
                Table::create()
                    .table(HormoneTrigger::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(HormoneTrigger::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(HormoneTrigger::HormoneId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(HormoneTrigger::TriggerId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-hormone_id")
                            .from(HormoneTrigger::Table, HormoneTrigger::HormoneId)
                            .to(Hormone::Table, Hormone::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-trigger_id")
                            .from(HormoneTrigger::Table, HormoneTrigger::TriggerId)
                            .to(Trigger::Table, Trigger::Id),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order of creation
        manager
            .drop_table(Table::drop().table(HormoneTrigger::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Hormone::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Trigger::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Description::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Hormone {
    Table,
    Id,
    Name,
    HormoneType,
    DescriptionId,
}

#[derive(Iden)]
enum Trigger {
    Table,
    Id,
    Description,
}

#[derive(Iden)]
enum HormoneTrigger {
    Table,
    Id,
    HormoneId,
    TriggerId,
}

#[derive(Iden)]
enum Description {
    Table,
    Id,
    Description,
    MetaDescription,
    InExcess,
    InNorm,
    InDeficiency,
}
