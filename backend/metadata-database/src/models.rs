use crate::MetadataDatabase;
use crate::data::{Model as ModelData, ModelCategory, ModelId, ModelType};
use crate::entity::ai_model;
use crate::entity::ai_model::Model;
use crate::entity::prelude::AiModel;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect,
};

impl MetadataDatabase {
    pub async fn add_ai_model(&self, model: ModelData) -> Result<ModelData, DbErr> {
        let model: Model = model.into();
        model
            .into_active_model()
            .insert(&self.conn)
            .await
            .map(ModelData::from)
    }

    pub async fn remove_ai_model(&self, id: ModelId) -> Result<(), DbErr> {
        AiModel::delete_by_id(id.into_inner())
            .exec(&self.conn)
            .await?;
        Ok(())
    }

    pub async fn find_ai_model_with_filter(
        &self,
        name_query: Option<String>,
        category: Option<ModelCategory>,
        model_type: Option<ModelType>,
        offset: usize,
        limit: usize,
    ) -> Result<(usize, Vec<ModelData>), DbErr> {
        let mut query = AiModel::find().order_by_asc(ai_model::Column::Name);
        if let Some(name_query) = name_query {
            query = query.filter(ai_model::Column::Name.contains(name_query));
        }
        if let Some(category) = category {
            query = query.filter(ai_model::Column::Category.eq(category));
        }
        if let Some(model_type) = model_type {
            query = query.filter(ai_model::Column::ModelType.eq(model_type));
        }

        let values = query
            .clone()
            .offset(offset as u64)
            .limit(limit as u64)
            .all(&self.conn)
            .await?;
        let count = query.count(&self.conn).await?;

        Ok((
            count as usize,
            values.into_iter().map(ModelData::from).collect(),
        ))
    }

    pub async fn find_ai_model_by_id(&self, id: ModelId) -> Result<Option<ModelData>, DbErr> {
        let value = AiModel::find_by_id(id.into_inner()).one(&self.conn).await?;
        Ok(value.map(ModelData::from))
    }

    pub async fn find_ai_model_by_name(&self, name: &str) -> Result<Option<ModelData>, DbErr> {
        let value = AiModel::find()
            .filter(ai_model::Column::FileName.eq(name))
            .one(&self.conn)
            .await?;
        Ok(value.map(ModelData::from))
    }
}
