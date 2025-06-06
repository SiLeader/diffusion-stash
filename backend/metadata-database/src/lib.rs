use sea_orm::Database;
use sea_orm::DatabaseConnection;

pub mod data;
mod entity;
mod generated_products;
mod models;

pub use sea_orm::DbErr;

#[derive(Clone)]
pub struct MetadataDatabase {
    conn: DatabaseConnection,
}

impl MetadataDatabase {
    pub async fn new(url: &str) -> Self {
        Self {
            conn: Database::connect(url).await.unwrap(),
        }
    }
}
