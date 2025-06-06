mod config;

use crate::config::Config;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use clap::Parser;
use log::{debug, info};
use metadata_database::MetadataDatabase;
use server::{DataStorage, DiffusionStashServer};

#[derive(Debug, Parser)]
struct Args {
    #[clap(
        help = "Path to config file",
        default_value = "/etc/diffusion-stash/config.yaml"
    )]
    config: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Starting diffusion stash backend server");

    let args = Args::parse();
    debug!("Command line arguments: {:?}", args);

    let config = Config::from_yaml(&args.config);

    let storage = DataStorage::new(
        config.storage.backend.create_storage(),
        config.storage.model_path.create_model_path_provider(),
        config.storage.product_path.create_product_path_provider(),
    );
    let database = MetadataDatabase::new(&config.database.url).await;

    let server = DiffusionStashServer::new(database, storage);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(|c| server.register(c))
    })
    .bind(&config.server.listen)
    .unwrap()
    .run()
    .await
    .unwrap();
}
