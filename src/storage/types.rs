use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Profile not found")]
    ProfileNotFound,
    #[error("Storage not available")]
    StorageUnavailable,
}

#[async_trait]
pub trait StorageBackend {
    async fn save_profile(&self, profile: &PlayerProfile) -> Result<(), StorageError>;
    async fn load_profile(&self, name: &str) -> Result<PlayerProfile, StorageError>;
    async fn save_progress(&self, progress: &GameProgress) -> Result<(), StorageError>;
    async fn load_progress(&self, profile_id: &str) -> Result<GameProgress, StorageError>;
}