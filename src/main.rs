use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};
use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::Dialogue,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};
use teloxide_core::types::Message;
use tokio;

mod models;
use models::{ChannelConfig, LoxoRC, LoxoUser, StorageType, StorageTypeCallback};

// -------------------------------------------------------- dialogue
type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    DialogueStart,
    DialogueStalk,
    DialogueSetStorage {
        storage_type: StorageType,
    },
}

// 1
async fn dialogue_start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log::info!("dialogue_start function used");

    bot.send_message(msg.chat.id, "Let's start! which user do u wanna stalk?")
        .await?;
    dialogue.update(State::DialogueStalk).await?;
    Ok(())
}

// 2
async fn dialogue_stalk(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log::info!("dialogue_stalk function used");

    let keyboard = InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback(
            "Google Drive".to_owned(),
            serde_json::to_string(&StorageTypeCallback {
                storage_type: StorageType::GoogleDrive,
            })
            .unwrap(),
        ),
        InlineKeyboardButton::callback(
            "Yandex360".to_owned(),
            serde_json::to_string(&StorageTypeCallback {
                storage_type: StorageType::Yandex360,
            })
            .unwrap(),
        ),
        InlineKeyboardButton::callback(
            "Local".to_owned(),
            serde_json::to_string(&StorageTypeCallback {
                storage_type: StorageType::Local,
            })
            .unwrap(),
        ),
    ]]);

    bot.send_message(msg.chat.id, "Okay, now select your storage:")
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

// 3.1
async fn handle_storage_callback(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    data: StorageTypeCallback,
) -> HandlerResult {
    log::info!("handle_storage_callback function used");

    if let Some(msg) = q.message {
        bot.send_message(
            msg.chat().id,
            format!("u chose {:?} storage.", data.storage_type),
        )
        .await?;

        // normaly this dialogue.update would be in dialogue_stalk
        dialogue
            .update(State::DialogueSetStorage {
                storage_type: data.storage_type,
            })
            .await?;
        Ok(())
    } else {
        // bot.send_message(q.unwrap(), "sth wrong with the button click.").await?;
        log::error!("something went wrong w buttons");
        Ok(())
    }
}

// 3.2 - only gets triggered by fn handle_storage_callback
// storage_type is available from State::DialogueSetStorage, bc handle_storage_callback sets State to DialogueSetStorage
async fn dialogue_set_storage(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    storage_type: StorageType,
) -> HandlerResult {
    log::info!(
        "dialogue_set_storage function used and {:#?} set as storage",
        storage_type
    );

    match msg.text().map(|text| text.parse::<u8>()) {
        Some(Ok(storage)) => {
            bot.send_message(msg.chat.id, "i thikn we're done at this point")
                .await?
        }
        _ => {
            bot.send_message(msg.chat.id, "idk send storage type or something.")
                .await?
        }
    };

    Ok(())
}

//--------------------------------------------------^ dialogue

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

// fn get_user_config(username: String) -> LoxoRC {
//     Ok(())
// }

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

    Command::repl(
        bot.clone(),
        move |bot: Bot, msg: Message, cmd: Command| async move { answer(bot, msg, cmd).await },
    )
    .await;

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::DialogueStart].endpoint(dialogue_start))
            .branch(dptree::case![State::DialogueStalk].endpoint(dialogue_stalk))
            .branch(
                dptree::case![State::DialogueSetStorage { storage_type }]
                    .endpoint(dialogue_set_storage),
            )
            .branch(Update::filter_callback_query().endpoint(handle_storage_callback)),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
