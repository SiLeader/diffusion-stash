use crate::Storage;
use crate::error::Error;
use std::path::PathBuf;
use tokio::fs::{create_dir_all, read, remove_file, write};

pub struct LocalStorage {
    base_directory: PathBuf,
}

impl LocalStorage {
    pub fn new(base_directory: PathBuf) -> Self {
        Self { base_directory }
    }
}

#[async_trait::async_trait]
impl Storage for LocalStorage {
    async fn save(&self, path: &str, data: &[u8]) -> Result<(), Error> {
        let full_path = self.base_directory.join(path);
        if let Some(parent) = full_path.parent() {
            create_dir_all(parent).await.map_err(Error::Io)?;
        }
        write(full_path, data).await.map_err(Error::Io)?;
        Ok(())
    }

    async fn load(&self, path: &str) -> Result<Vec<u8>, Error> {
        let full_path = self.base_directory.join(path);
        match read(&full_path).await {
            Ok(data) => Ok(data),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(Error::NotFound),
            Err(e) => Err(Error::Io(e)),
        }
    }

    async fn delete(&self, path: &str) -> Result<(), Error> {
        let full_path = self.base_directory.join(path);
        remove_file(full_path).await.map_err(Error::Io)
    }
}
