use crate::models::{LoxoRC, LoxoUser, ChannelConfig, StorageType, StorageTypeCallback};
use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::Dialogue,
    prelude::*,
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message, ChatId, Recipient, User},
};
use chrono::Utc;
use std::{env, fs::{File, OpenOptions}, io::{self, Write}, path::{PathBuf}};

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    DialogueStart,
    DialogueStalk {
        client_chat_id: ChatId,
        client_user_id: Option<User>
    },
    DialogueSetStorage {
        client_chat_id: ChatId,
        client_user_id: Option<User>,
        target_username: Recipient
    }
}

// 1
pub async fn dialogue_start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log::info!("dialogue_start function used");

    let client_chat_id: ChatId  = dialogue.chat_id();
    let client_user_id: Option<User> = msg.from;    // gotta match None later

    bot.send_message(msg.chat.id, format!("Let's start! which user do u wanna stalk?\n\nCurrent chat id is {:?}", client_chat_id)).await?;
    dialogue.update(State::DialogueStalk { client_chat_id, client_user_id }).await?;
    Ok(())
}

// 2
pub async fn dialogue_stalk(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log::info!("dialogue_stalk function used");

    // retrieving state xdd [1]
    let current_state = dialogue.get().await?.unwrap();
    let (client_chat_id, client_user_id) = if let State::DialogueStalk { client_chat_id, client_user_id } = current_state {
        (client_chat_id, client_user_id)
    } else {
        return Ok(());
    };

    // checking if user input is a Recipient::ChannelUsername variant then proceeding w/button input
    if let Some(username) = msg.text() {
        if username.contains(' ') {
            bot.send_message(msg.chat.id, "username can't contain spaces").await?;
            return Ok(());
        }

        let target_username = Recipient::ChannelUsername(username.to_string());

        // button input
        let keyboard = InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::callback(
                "Google Drive".to_owned(),
                serde_json::to_string(&StorageTypeCallback {
                    storage_type: StorageType::GoogleDrive,
                })
                .unwrap(),
            ),
            InlineKeyboardButton::callback(
                "YandexDisk".to_owned(),
                serde_json::to_string(&StorageTypeCallback {
                    storage_type: StorageType::YandexDisk,
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
    
        bot.send_message(msg.chat.id, format!("Okay, now select storage to save {:?}'s data to", target_username))
            .reply_markup(keyboard)
            .await?;
    
        dialogue.update(State::DialogueSetStorage { client_chat_id, client_user_id, target_username }).await?;
    } else {
        bot.send_message(msg.chat.id, "sry, invalid channel username, try again").await?;
    };

    Ok(())
}

// 3, gets triggered on button click
pub async fn dialogue_storage_callback(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
) -> HandlerResult {
    log::info!("dialogue_storage_callback function used");

    // retrieving state xdd [2]
    let current_state = dialogue.get().await?.unwrap();
    let (client_chat_id, client_user_id, target_username) = 
        if let State::DialogueSetStorage { client_chat_id, client_user_id, target_username } = current_state {
            (client_chat_id, client_user_id, target_username)
        } else {
            return Ok(());
        };


    if let Some(msg) = q.message {
        if let Some(data_str) = q.data {
            match serde_json::from_str::<StorageTypeCallback>(&data_str) {
                Ok(data) => {
                    let storage = data.storage_type;
                    bot.send_message(
                        msg.chat().id,
                        format!("u chose {:?} storage. saving your config.\n\ncurrent chat id: {:?}, stalking {:?}", storage, client_chat_id, target_username),
                    )
                    .await?;

                // WIP
                save_loxorc(client_chat_id, client_user_id, target_username, storage).await?;

                }
                Err(err) => {
                    log::error!("Failed to parse StorageTypeCallback: {:?}", err);
                    bot.send_message(msg.chat().id, "Invalid selection.").await?;
                }
            }
        }
        Ok(())
    } else {
        log::error!("something went wrong w buttons (no message)");
        Ok(())
    }
}

// WIP per-client config that will end up in mongo
pub async fn save_loxorc(client_chat_id: ChatId, client_user_id: Option<User>, target_username: Recipient, storage: StorageType) -> io::Result<()> {
    
    let loxo_rc_path: PathBuf = env::temp_dir().join(PathBuf::from(format!("{}_config.json", client_chat_id)));
    let mut loxo_rc_file: File = OpenOptions::new().create(true).append(true).open(&loxo_rc_path)?;

    let loxo_rc: LoxoRC = LoxoRC {
        default_storage: None,
        loxo_users: vec![LoxoUser {
            client_chat_id: client_chat_id,
            client_user_id: client_user_id,
            target_username: target_username,
            target_channel_config: ChannelConfig {
                storage,
                last_update: Some(Utc::now().timestamp() as i32),     // placeholder! don't forget to eventually start updating it 
                update_every: Some(3600),                             // placeholder!
            },
        }],
    };

    let loxo_json = serde_json::to_string_pretty(&loxo_rc.loxo_users)?; // the one goes to mongo
    loxo_rc_file.write_all(loxo_json.as_bytes()).unwrap();

    Ok(())
}
