use serde::{Deserialize, Serialize};
pub mod storage;
pub mod cli;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProfile {
    name: String,
    email: Option<String>,
    is_premium: bool,
    last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameProgress {
    profile_id: String,
    data: serde_json::Value,
    timestamp: chrono::DateTime<chrono::Utc>,
}
