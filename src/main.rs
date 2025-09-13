use chrono::Local;
use std::{env, fs::{OpenOptions,File}, io::{Write, IoSlice}, sync::Arc, path::Path};
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide_core::types::Message;
use tokio;

mod models;
use models::{LoxoRC, LoxoRCConfig, StorageType};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Display help.")]
    Help,
    #[command(description = "Stalk a channel.")]
    Stalk { channel_id: String },
    #[command(description = "Set default cloud storage.")]
    Storage { storage: String },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Stalk { channel_id } => {
            bot.send_message(msg.chat.id, format!("you wanna stalk: {channel_id}"))
                .await?

            
        }
        Command::Storage { storage } => {
            bot.send_message(msg.chat.id, format!("selected storage: {storage}"))
                .await?
        }
    };

    Ok(())
}

fn init_rc() -> std::io::Result<File> {
    let home_dir = dirs::home_dir().expect("no home dir found");
    let rc_path = home_dir.join(".loxosceles.rc");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(rc_path)?;

    Ok(file)

}

fn log_request(message: &str, username: Option<&str>) -> std::io::Result<()> {
    let log_path = Path::new("/var/log/loxosceles.log");

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

#[tokio::main]
async fn main() {
    unsafe {
        env::set_var(
            "TELOXIDE_TOKEN",
            "8450777546:AAHJbPOVlzFnSnoSuhCLI8ql0R81BHEt-N8",
        );
    }
    pretty_env_logger::init();
    log::info!("Starting loxosceles bot...");
    let bot = Bot::from_env();

    // === test input
    let user1 = LoxoRC {
        channel_id: String::from("some_channel"),
        config: LoxoRCConfig {
            last_update: 10,
            update_every: 20,
            storage: StorageType::GoogleDrive
        }
    };
    let json_user1 = serde_json::to_string_pretty(&user1).expect("failed to serialize input");
    let mut rc_file:Result<File, std::io::Error> = init_rc();

    let mut rc_result = match rc_file {
        // Ok(mut file) => writeln!(file, "{:#?}", json_user1),
        Ok(mut file) => file.write_all(json_user1.as_bytes()).unwrap(),
        Err(e) => panic!("{e}"), 
    };
 
    // === test input ^


    Command::repl(
        bot,
        move |bot: Bot, msg: Message, cmd: Command| async move { answer(bot, msg, cmd).await },
    )
    .await;
}
