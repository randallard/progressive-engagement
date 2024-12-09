use super::types::{StorageBackend, StorageError};
use async_trait::async_trait;
use std::path::PathBuf;

pub struct LocalStorage {
    base_path: PathBuf,
}

impl LocalStorage {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }
}

#[async_trait]
impl StorageBackend for LocalStorage {
    async fn save_profile(&self, profile: &PlayerProfile) -> Result<(), StorageError> {
        let path = self.base_path.join(format!("{}.profile", profile.name));
        let json = serde_json::to_string_pretty(profile)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }

    async fn load_profile(&self, name: &str) -> Result<PlayerProfile, StorageError> {
        let path = self.base_path.join(format!("{}.profile", name));
        let json = tokio::fs::read_to_string(path).await?;
        let profile = serde_json::from_str(&json)?;
        Ok(profile)
    }

    // Implement save_progress and load_progress similarly
}