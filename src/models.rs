use serde::{Deserialize, Serialize};
use strum::EnumString;
use teloxide::types::{Recipient, ChatId, User};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct LoxoRC {
    pub default_storage: Option<StorageType>,
    pub loxo_users: Vec<LoxoUserConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct LoxoUserConfig {
    pub client_chat_id: ChatId,
    pub client_user_id: Option<User>,
    pub target_username: Recipient,
    pub target_channel_config: ChannelConfig,
}

// TODO: set dates as PrimitiveDateTime,
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ChannelConfig {
    pub storage: StorageType,
    pub last_update: Option<i32>,
    pub update_every: Option<i32>,
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
