use crate::endpoints::register_endpoints;
use crate::storage::DataStorage;
use ::storage::Storage;
use actix_web::web::{Data, ServiceConfig};
use metadata_database::MetadataDatabase;

mod endpoints;
mod storage;

#[derive(Clone)]
pub struct DiffusionStashServer {
    database: MetadataDatabase,
    storage: DataStorage,
}

impl DiffusionStashServer {
    pub fn new<S>(database: MetadataDatabase, storage: S) -> Self
    where
        S: 'static + Storage + Send + Sync,
    {
        Self {
            database,
            storage: DataStorage::new(storage),
        }
    }
}

impl DiffusionStashServer {
    pub fn register(&self, config: &mut ServiceConfig) {
        config
            .app_data(Data::new(self.database.clone()))
            .app_data(Data::new(self.storage.clone()));
        register_endpoints(config);
    }
}
