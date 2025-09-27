use core::fmt;
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use strum::EnumString;
use teloxide::types::{ChatId, Recipient};
use time::PrimitiveDateTime;

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
    pub last_update: i32,
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
