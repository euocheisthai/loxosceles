use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use core::fmt;
// use chrono::{DateTime, Utc, NaiveTime, NaiveDate, NaiveDateTime};
// use chrono_simpletz::{TimeZoneZst, known_timezones::UtcP1};
use time::PrimitiveDateTime;
use strum::EnumString;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] 
pub struct LoxoRC {
    pub default_storage: Option<StorageType>,
    pub loxo_users: Vec<LoxoUser>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] 
pub struct LoxoUser {
    pub channel_id: String,
    pub channel_config: ChannelConfig
}

// TODO: set dates as PrimitiveDateTime,
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] 
pub struct ChannelConfig {
    pub storage: StorageType,
    pub last_update: i32,
    pub update_every: i32
}

#[derive(Debug, Serialize, Deserialize, Clone, EnumString)]
#[serde(rename_all = "lowercase")] 
pub enum StorageType {
    GoogleDrive,
    Yandex360,
    Local
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageTypeCallback {
    pub storage_type: StorageType,
}
