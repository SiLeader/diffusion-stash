use crate::path::PathProvider;
use metadata_database::data::{GeneratedProduct, Model};
use std::sync::Arc;
use storage::{Error, Storage};

#[derive(Clone)]
pub struct DataStorage {
    storage: Arc<dyn Storage + Send + Sync>,
    model_path_provider: Arc<dyn PathProvider<Model>>,
    product_path_provider: Arc<dyn PathProvider<GeneratedProduct>>,
}

impl DataStorage {
    pub fn new(
        storage: Arc<dyn Storage + Send + Sync>,
        model_path_provider: Arc<dyn PathProvider<Model>>,
        product_path_provider: Arc<dyn PathProvider<GeneratedProduct>>,
    ) -> Self {
        Self {
            storage,
            model_path_provider,
            product_path_provider,
        }
    }
}

impl DataStorage {
    pub(crate) async fn save_model(&self, model: &Model, data: &[u8]) -> Result<(), Error> {
        let path = self.model_path_provider.get_path(model);
        self.storage.save(&path, data).await
    }

    pub(crate) async fn load_model(&self, model: &Model) -> Result<Vec<u8>, Error> {
        let path = self.model_path_provider.get_path(model);
        self.storage.load(&path).await
    }

    pub(crate) async fn save_product(
        &self,
        product: &GeneratedProduct,
        data: &[u8],
    ) -> Result<(), Error> {
        let path = self.product_path_provider.get_path(product);
        self.storage.save(&path, data).await
    }

    pub(crate) async fn load_product(&self, product: &GeneratedProduct) -> Result<Vec<u8>, Error> {
        let path = self.product_path_provider.get_path(product);
        self.storage.load(&path).await
    }
}
