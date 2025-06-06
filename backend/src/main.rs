mod config;

use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Starting diffusion stash backend server");
}
