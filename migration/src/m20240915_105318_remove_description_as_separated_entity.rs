use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240915_105318_remove_description_as_separated_entity"
    }
}


#[derive(Iden)]
enum Hormones {
    Table,
    Id,
    Name,
    HormoneType,
    // Fields to be added
    Description,
    MetaDescription,
    InExcess,
    InNorm,
    InDeficiency,
    // Fields to be dropped
    DescriptionId,
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

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Hormones::Table)
                    .drop_foreign_key(Alias::new("fk-description_id"))
                    .drop_column(Hormones::DescriptionId)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hormones::Table)
                    .add_column(ColumnDef::new(Hormones::Description).string().null())
                    .add_column(ColumnDef::new(Hormones::MetaDescription).string().null())
                    .add_column(ColumnDef::new(Hormones::InExcess).string().null())
                    .add_column(ColumnDef::new(Hormones::InNorm).string().null())
                    .add_column(ColumnDef::new(Hormones::InDeficiency).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Descriptions::Table).to_owned())
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Step 1: Re-create the 'descriptions' table
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
                    .col(ColumnDef::new(Descriptions::Description).string().null())
                    .col(ColumnDef::new(Descriptions::MetaDescription).string().null())
                    .col(ColumnDef::new(Descriptions::InExcess).string().null())
                    .col(ColumnDef::new(Descriptions::InNorm).string().null())
                    .col(ColumnDef::new(Descriptions::InDeficiency).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hormones::Table)
                    .add_column(ColumnDef::new(Hormones::DescriptionId).integer().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hormones::Table)
                    .add_foreign_key(
                        &TableForeignKey::new()
                            .name("fk-description_id")
                            .from_tbl(Hormones::Table)
                            .from_col(Hormones::DescriptionId)
                            .to_tbl(Descriptions::Table)
                            .to_col(Descriptions::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Hormones::Table)
                    .drop_column(Hormones::Description)
                    .drop_column(Hormones::MetaDescription)
                    .drop_column(Hormones::InExcess)
                    .drop_column(Hormones::InNorm)
                    .drop_column(Hormones::InDeficiency)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
