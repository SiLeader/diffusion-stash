use crate::Storage;
use crate::error::Error;
use std::path::Path;
use tokio::fs::{create_dir_all, read, remove_file, rename, write};

pub struct LocalStorage;

#[async_trait::async_trait]
impl Storage for LocalStorage {
    async fn save(&self, path: &Path, data: &[u8]) -> Result<(), Error> {
        if let Some(parent) = path.parent() {
            create_dir_all(parent).await.map_err(Error::Io)?;
        }
        write(path, data).await.map_err(Error::Io)?;
        Ok(())
    }

    async fn load(&self, path: &Path) -> Result<Vec<u8>, Error> {
        match read(&path).await {
            Ok(data) => Ok(data),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(Error::NotFound),
            Err(e) => Err(Error::Io(e)),
        }
    }

    async fn delete(&self, path: &Path) -> Result<(), Error> {
        remove_file(path).await.map_err(Error::Io)
    }

    async fn move_file(&self, from: &Path, to: &Path) -> Result<(), Error> {
        rename(from, to).await.map_err(Error::Io)
    }
}
