use metadata_database::data::{GeneratedProductId, ModelId};
use std::sync::Arc;
use storage::{Error, Storage};

#[derive(Clone)]
pub(crate) struct DataStorage {
    storage: Arc<dyn Storage + Send + Sync>,
}

impl DataStorage {
    pub(crate) fn new<S>(s: S) -> Self
    where
        S: 'static + Storage + Send + Sync,
    {
        Self {
            storage: Arc::new(s),
        }
    }
}

fn model_path(id: &ModelId) -> String {
    format!("/models/{}", id)
}

fn product_path(id: &GeneratedProductId) -> String {
    format!("/products/{}", id)
}

impl DataStorage {
    pub(crate) async fn save_model(&self, id: &ModelId, data: &[u8]) -> Result<(), Error> {
        let path = model_path(id);
        self.storage.save(&path, data).await
    }

    pub(crate) async fn load_model(&self, id: &ModelId) -> Result<Vec<u8>, Error> {
        let path = model_path(id);
        self.storage.load(&path).await
    }

    pub(crate) async fn save_product(
        &self,
        id: &GeneratedProductId,
        data: &[u8],
    ) -> Result<(), Error> {
        let path = product_path(id);
        self.storage.save(&path, data).await
    }

    pub(crate) async fn load_product(&self, id: &GeneratedProductId) -> Result<Vec<u8>, Error> {
        let path = product_path(id);
        self.storage.load(&path).await
    }
}
