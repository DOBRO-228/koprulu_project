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
                    .table(Triggers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Triggers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Triggers::Description).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Create descriptions table
        manager
            .create_table(
                Table::create()
                    .table(Descriptions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Descriptions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Descriptions::Description)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Descriptions::MetaDescription)
                            .string()
                            .null(),
                    )
                    .col(ColumnDef::new(Descriptions::InExcess).string().null())
                    .col(ColumnDef::new(Descriptions::InNorm).string().null())
                    .col(ColumnDef::new(Descriptions::InDeficiency).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Hormones::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Hormones::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Hormones::Name).string().not_null())
                    .col(ColumnDef::new(Hormones::HormoneType).string().not_null())
                    .col(ColumnDef::new(Hormones::DescriptionId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-description_id")
                            .from(Hormones::Table, Hormones::DescriptionId)
                            .to(Descriptions::Table, Descriptions::Id),
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
                            .to(Hormones::Table, Hormones::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-trigger_id")
                            .from(HormoneTrigger::Table, HormoneTrigger::TriggerId)
                            .to(Triggers::Table, Triggers::Id),
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
            .drop_table(Table::drop().table(Hormones::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Triggers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Descriptions::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Hormones {
    Table,
    Id,
    Name,
    HormoneType,
    DescriptionId,
}

#[derive(Iden)]
enum Triggers {
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
enum Descriptions {
    Table,
    Id,
    Description,
    MetaDescription,
    InExcess,
    InNorm,
    InDeficiency,
}
