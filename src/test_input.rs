use chrono::Local;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};
use teloxide::types::{Recipient};

use crate::models::{ChannelConfig, LoxoUser, StorageType};

pub fn test_input() -> () {
    let rc_file: Result<File, std::io::Error> = init_rc();
    let user1 = LoxoUser {
        target_channel_username: Recipient::ChannelUsername(String::from("some_username")),
        target_channel_config: ChannelConfig {
            last_update: 10,
            update_every: 20,
            storage: StorageType::GoogleDrive,
        },
    };
    let json_user1 = serde_json::to_string_pretty(&user1).expect("failed to serialize input");
    let mut rc_file: Result<File, std::io::Error> = init_rc();

    let mut rc_result = match rc_file {
        Ok(mut file) => file.write_all(json_user1.as_bytes()).unwrap(),
        Err(e) => panic!("{e}"),
    };  
}

pub fn init_rc() -> std::io::Result<File> {
    let home_dir: PathBuf = dirs::home_dir().expect("no home dir found");
    let rc_path: PathBuf = home_dir.join(".loxosceles.rc");

    let mut file: File = OpenOptions::new().create(true).append(true).open(rc_path)?;

    Ok(file)
}

pub fn log_request(message: &str, username: Option<&str>) -> std::io::Result<()> {
    let log_path: &Path = Path::new("/var/log/loxosceles.log");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S");

    match username {
        Some(name) => writeln!(file, "[{timestamp}] @{name}:\n{message}")?,
        None => writeln!(file, "[{timestamp}]\n{message}")?,
    }

    Ok(())
}
