use crate::dialogues::HandlerResult;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};
use teloxide::{
    dispatching::UpdateHandler,
    dispatching::{dialogue, dialogue::InMemStorage},
    prelude::*,
    utils::command::BotCommands,
};
use teloxide_core::types::Message;
use tokio;

mod models;
use models::{ChannelConfig, LoxoRC, LoxoUser, StorageType, StorageTypeCallback};
mod dialogues;
use dialogues::{
    State, dialogue_set_storage, dialogue_stalk, dialogue_start, handle_storage_callback,
};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Display help.")]
    Help,
    // command /start, should kickstart a dialogue
    #[command(description = "Stalk a channel.")]
    Stalk,
    #[command(description = "Set default cloud storage.")]
    Storage { storage: String },
}

async fn display_help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

// placeholder
async fn set_default_storage(bot: Bot, msg: Message, storage: String) -> HandlerResult {
    bot.send_message(msg.chat.id, format!("Default storage set to: {storage}"))
        .await?;
    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Unable to handle the message. Type /help to see the usage.",
    )
    .await?;
    Ok(())
}

// ---------------------------- test input section
fn test_input() -> () {
    let mut rc_file: Result<File, std::io::Error> = init_rc();
    let user1 = LoxoUser {
        channel_id: String::from("some_channel"),
        config: ChannelConfig {
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

fn init_rc() -> std::io::Result<File> {
    let home_dir = dirs::home_dir().expect("no home dir found");
    let rc_path = home_dir.join(".loxosceles.rc");

    let mut file = OpenOptions::new().create(true).append(true).open(rc_path)?;

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

// ---------------------------- test input section

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(display_help))
        // this one kickstarts the main dialogue
        .branch(case![Command::Stalk].endpoint(dialogue_start))
        .branch(case![Command::Storage { storage }].endpoint(set_default_storage));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        // if state is currently at DialogueStalk, move to dialogue_stalk
        .branch(case![State::DialogueStalk].endpoint(dialogue_stalk))
        // if state is currently DialogueSetStorage, move to dialogue_set_storage
        .branch(case![State::DialogueSetStorage { storage_type }].endpoint(dialogue_set_storage))
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler =
        Update::filter_callback_query().branch(dptree::endpoint(handle_storage_callback));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

#[tokio::main]
async fn main() {
    unsafe {
        if let Ok(token_value) = env::var("LOXOSCELES_TOKEN") {
            env::set_var("TELOXIDE_TOKEN", token_value);
        } else {
            eprintln!("forgot to set LOXOSCELES_TOKEN...");
        }
    }
    pretty_env_logger::init();
    log::info!("Starting loxosceles bot...");
    let bot = Bot::from_env();

    // === test input
    let mut rc_file: Result<File, std::io::Error> = init_rc();
    test_input();
    // === test input ^

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
