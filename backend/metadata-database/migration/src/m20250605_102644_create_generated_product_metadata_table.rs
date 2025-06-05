use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GeneratedProduct::Table)
                    .if_not_exists()
                    .col(pk_uuid(GeneratedProduct::Id))
                    .col(string(GeneratedProduct::Name))
                    .col(string(GeneratedProduct::MimeType))
                    .col(string_null(GeneratedProduct::PositivePrompt))
                    .col(string_null(GeneratedProduct::NegativePrompt))
                    .col(string_null(GeneratedProduct::SamplerName))
                    .col(string_null(GeneratedProduct::SchedulerName))
                    .col(integer_null(GeneratedProduct::StepCount))
                    .col(float_null(GeneratedProduct::CfgScale))
                    .col(integer_null(GeneratedProduct::Seed))
                    .col(timestamp(GeneratedProduct::CreatedAt))
                    .col(timestamp(GeneratedProduct::UpdatedAt))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GeneratedProduct::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum GeneratedProduct {
    Table,
    Id,
    Name,
    MimeType,
    PositivePrompt,
    NegativePrompt,
    SamplerName,
    SchedulerName,
    StepCount,
    CfgScale,
    Seed,
    CreatedAt,
    UpdatedAt,
}
