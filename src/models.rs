use serde::{Deserialize, Serialize};
use strum::EnumString;
use teloxide::types::{Recipient};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct LoxoRC {
    pub default_storage: Option<StorageType>,
    pub loxo_users: Vec<LoxoUser>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct LoxoUser {
    pub target_channel_username: Recipient,
    pub target_channel_config: ChannelConfig,
}

// TODO: set dates as PrimitiveDateTime,
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ChannelConfig {
    pub storage: StorageType,
    pub last_update: i32,               // actually chrono::Utc
    pub update_every: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum StorageType {
    GoogleDrive,
    YandexDisk,
    Local,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageTypeCallback {
    pub storage_type: StorageType,
}
