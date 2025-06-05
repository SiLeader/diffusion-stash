use crate::m20250605_102218_create_model_metadata_table::AiModel;
use crate::m20250605_102644_create_generated_product_metadata_table::GeneratedProduct;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GeneratedProductModelAssociation::Table)
                    .if_not_exists()
                    .col(uuid(GeneratedProductModelAssociation::GeneratedProductId))
                    .col(uuid(GeneratedProductModelAssociation::ModelId))
                    .primary_key(
                        Index::create()
                            .col(GeneratedProductModelAssociation::GeneratedProductId)
                            .col(GeneratedProductModelAssociation::ModelId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(GeneratedProductModelAssociation::ModelId)
                            .to_tbl(AiModel::Table)
                            .to_col(AiModel::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(GeneratedProductModelAssociation::GeneratedProductId)
                            .to_tbl(GeneratedProduct::Table)
                            .to_col(GeneratedProduct::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(GeneratedProductModelAssociation::Table)
                    .if_not_exists()
                    .name("idx_gpm_association_model")
                    .col(GeneratedProductModelAssociation::ModelId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(GeneratedProductModelAssociation::Table)
                    .if_not_exists()
                    .name("idx_gpm_association_generated_product")
                    .col(GeneratedProductModelAssociation::GeneratedProductId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .table(GeneratedProductModelAssociation::Table)
                    .name("idx_gpm_association_model")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(GeneratedProductModelAssociation::Table)
                    .name("idx_gpm_association_generated_product")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(GeneratedProductModelAssociation::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum GeneratedProductModelAssociation {
    Table,
    GeneratedProductId,
    ModelId,
}
