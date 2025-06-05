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
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}
