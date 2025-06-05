mod error;
pub mod local;

pub use error::*;

#[async_trait::async_trait]
pub trait Storage {
    async fn save(&self, path: &str, data: &[u8]) -> Result<(), Error>;
    async fn load(&self, path: &str) -> Result<Vec<u8>, Error>;
    async fn delete(&self, path: &str) -> Result<(), Error>;
}
