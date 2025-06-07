use crate::MetadataDatabase;
use crate::data::ModelId;
use crate::data::{GeneratedProduct as GeneratedProductData, GeneratedProductId};
use crate::entity::prelude::{AiModel, GeneratedProduct, GeneratedProductModelAssociation};
use crate::entity::{ai_model, generated_product, generated_product_model_association};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, IntoActiveModel, ModelTrait, PaginatorTrait,
    QueryFilter, QuerySelect, TransactionError, TransactionTrait,
};

impl MetadataDatabase {
    pub async fn add_generated_product(
        &self,
        model: GeneratedProductData,
    ) -> Result<GeneratedProductData, DbErr> {
        let (product, models) = model.into();
        let res = self
            .conn
            .transaction::<_, (generated_product::Model, Vec<ai_model::Model>), DbErr>(|tx| {
                Box::pin(async move {
                    let product = product.into_active_model().insert(tx).await?;
                    if !models.is_empty() {
                        GeneratedProductModelAssociation::insert_many(
                            models
                                .iter()
                                .map(|m| generated_product_model_association::Model {
                                    generated_product_id: product.id.clone(),
                                    model_id: m.id,
                                })
                                .map(IntoActiveModel::into_active_model),
                        )
                        .exec(tx)
                        .await?;
                    }
                    Ok((product, models))
                })
            })
            .await;

        match res {
            Ok(value) => Ok(value.into()),
            Err(e) => match e {
                TransactionError::Connection(e) => Err(e),
                TransactionError::Transaction(e) => Err(e),
            },
        }
    }

    pub async fn remove_generated_product(&self, id: GeneratedProductId) -> Result<(), DbErr> {
        GeneratedProduct::delete_by_id(id.into_inner())
            .exec(&self.conn)
            .await?;
        Ok(())
    }

    pub async fn find_generated_product_by_model(
        &self,
        model_id: ModelId,
        offset: usize,
        limit: usize,
    ) -> Result<(usize, Vec<GeneratedProductData>), DbErr> {
        let products = GeneratedProduct::find()
            .find_with_related(AiModel)
            .filter(ai_model::Column::Id.eq(model_id.clone().into_inner()))
            .offset(offset as u64)
            .limit(limit as u64)
            .all(&self.conn)
            .await?;
        let count = GeneratedProductModelAssociation::find()
            .filter(generated_product_model_association::Column::ModelId.eq(model_id.into_inner()))
            .count(&self.conn)
            .await?;

        Ok((
            count as usize,
            products.into_iter().map(|p| p.into()).collect(),
        ))
    }

    pub async fn find_generated_product_by_id(
        &self,
        id: GeneratedProductId,
    ) -> Result<Option<GeneratedProductData>, DbErr> {
        let Some(product) = GeneratedProduct::find_by_id(id.into_inner())
            .one(&self.conn)
            .await?
        else {
            return Ok(None);
        };
        let models = product.find_related(AiModel).all(&self.conn).await?;
        Ok(Some((product, models).into()))
    }
}
