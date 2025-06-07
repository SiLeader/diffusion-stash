use crate::sea_orm::DatabaseBackend;
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() == DatabaseBackend::Postgres {
            manager
                .create_type(
                    Type::create()
                        .as_enum(ModelBaseEnum)
                        .values(ModelBase::iter())
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
        }
        manager
            .create_table(
                Table::create()
                    .table(AiModel::Table)
                    .if_not_exists()
                    .col(pk_uuid(AiModel::Id))
                    .col(string(AiModel::Name))
                    .col(string_uniq(AiModel::FileName))
                    .col(string(AiModel::Description))
                    .col(enumeration_null(
                        AiModel::BaseModel,
                        ModelBaseEnum,
                        ModelBase::iter(),
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
                    .name("idx_model_metadata_name")
                    .col(AiModel::Name)
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
                    .name("idx_model_metadata_base_model")
                    .col(AiModel::BaseModel)
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
            .drop_index(
                Index::drop()
                    .table(AiModel::Table)
                    .name("idx_model_metadata_name")
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
struct ModelBaseEnum;

#[derive(DeriveIden)]
struct ModelTypeEnum;

#[derive(Iden, EnumIter)]
enum ModelBase {
    StableDiffusion15,
    StableDiffusionXL,
    PonyDiffusion,
    Illustrious,
    StableDiffusion30,
    StableDiffusion35,
    Flux1S,
    Flux1D,
}

#[derive(Iden, EnumIter)]
enum ModelType {
    Checkpoint,
    Embedding,
    Lora,
    Lycoris,
    ControlNet,
    Upscaler,
    Vae,
}

#[derive(DeriveIden)]
pub enum AiModel {
    Table,
    Id,
    Name,
    Description,
    FileName,
    BaseModel,
    ModelType,
    CreatedAt,
    UpdatedAt,
}
