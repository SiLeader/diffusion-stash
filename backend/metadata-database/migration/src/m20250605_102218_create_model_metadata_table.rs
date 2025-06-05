use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ModelCategoryEnum)
                    .values(ModelCategory::iter())
                    .to_owned(),
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(ModelTypeEnum)
                    .values(ModelType::iter())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(AiModel::Table)
                    .if_not_exists()
                    .col(pk_uuid(AiModel::Id))
                    .col(string(AiModel::Name))
                    .col(enumeration_null(
                        AiModel::Category,
                        ModelCategoryEnum,
                        ModelCategory::iter(),
                    ))
                    .col(enumeration_null(
                        AiModel::ModelType,
                        ModelTypeEnum,
                        ModelType::iter(),
                    ))
                    .col(timestamp(AiModel::CreatedAt))
                    .col(timestamp(AiModel::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(AiModel::Table)
                    .if_not_exists()
                    .name("idx_model_metadata_model_type")
                    .col(AiModel::ModelType)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(AiModel::Table)
                    .if_not_exists()
                    .name("idx_model_metadata_category")
                    .col(AiModel::Category)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .table(AiModel::Table)
                    .name("idx_model_metadata_model_type")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(AiModel::Table)
                    .name("idx_model_metadata_category")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(AiModel::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
struct ModelCategoryEnum;

#[derive(DeriveIden)]
struct ModelTypeEnum;

#[derive(Iden, EnumIter)]
enum ModelCategory {
    Checkpoint,
    Embedding,
    Lora,
    Lycoris,
    ControlNet,
    Upscaler,
    Vae,
}

#[derive(Iden, EnumIter)]
enum ModelType {
    StableDiffusion15,
    StableDiffusionXL,
    PonyDiffusion,
    Illustrious,
    StableDiffusion30,
    StableDiffusion35,
    Flux1S,
    Flux1D,
}

#[derive(DeriveIden)]
pub enum AiModel {
    Table,
    Id,
    Name,
    Category,
    ModelType,
    CreatedAt,
    UpdatedAt,
}
