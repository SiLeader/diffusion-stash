pub use sea_orm_migration::prelude::*;

mod m20250605_102218_create_model_metadata_table;
mod m20250605_102644_create_generated_product_metadata_table;
mod m20250605_103636_create_generated_product_model_association_table;
mod m20250612_110830_create_datetime_index;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250605_102218_create_model_metadata_table::Migration),
            Box::new(m20250605_102644_create_generated_product_metadata_table::Migration),
            Box::new(m20250605_103636_create_generated_product_model_association_table::Migration),
            Box::new(m20250612_110830_create_datetime_index::Migration),
        ]
    }
}
