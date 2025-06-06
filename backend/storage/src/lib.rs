mod error;
pub mod local;

pub use error::*;
use std::path::Path;

#[async_trait::async_trait]
pub trait Storage {
    async fn save(&self, path: &Path, data: &[u8]) -> Result<(), Error>;
    async fn load(&self, path: &Path) -> Result<Vec<u8>, Error>;
    async fn delete(&self, path: &Path) -> Result<(), Error>;
    async fn move_file(&self, from: &Path, to: &Path) -> Result<(), Error>;
}
