use crate::m20250605_102218_create_model_metadata_table::AiModel;
use crate::m20250605_102644_create_generated_product_metadata_table::GeneratedProduct;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .table(AiModel::Table)
                    .name("idx_ai_model_created_at")
                    .if_not_exists()
                    .col(AiModel::CreatedAt)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(AiModel::Table)
                    .name("idx_ai_model_updated_at")
                    .if_not_exists()
                    .col(AiModel::UpdatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(GeneratedProduct::Table)
                    .name("idx_generated_product_created_at")
                    .if_not_exists()
                    .col(GeneratedProduct::CreatedAt)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(GeneratedProduct::Table)
                    .name("idx_generated_product_updated_at")
                    .if_not_exists()
                    .col(GeneratedProduct::UpdatedAt)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_ai_model_created_at").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_ai_model_updated_at").to_owned())
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_generated_product_created_at")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_generated_product_updated_at")
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
