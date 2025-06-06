use crate::endpoints::register_endpoints;
use actix_web::web::{Data, ServiceConfig};
use metadata_database::MetadataDatabase;

mod endpoints;
pub mod path;
mod storage;

pub use storage::*;

#[derive(Clone)]
pub struct DiffusionStashServer {
    database: MetadataDatabase,
    storage: DataStorage,
}

impl DiffusionStashServer {
    pub fn new(database: MetadataDatabase, storage: DataStorage) -> Self {
        Self { database, storage }
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
