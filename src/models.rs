use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
// use chrono::{DateTime, Utc, NaiveTime, NaiveDate, NaiveDateTime};
// use chrono_simpletz::{TimeZoneZst, known_timezones::UtcP1};
use time::PrimitiveDateTime;



#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] 
pub struct LoxoRC {
    pub channel_id: String,
    pub config: LoxoRCConfig
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] 
pub struct LoxoRCConfig {
    // last_update: PrimitiveDateTime,
    pub last_update: i32,
    pub update_every: i32,
    pub storage: StorageType
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] 
pub enum StorageType {
    GoogleDrive,
    Yandex360,
    Local
}