use crate::models::{StorageType, StorageTypeCallback};
use serde::{Deserialize, Serialize};
use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::Dialogue,
    prelude::*,
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message, ChatId, Recipient},
};

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    DialogueStart,
    DialogueStalk {
        client_chat_id: ChatId,
    },
    DialogueSetStorage {
        client_chat_id: ChatId,
        target_channel_username: Recipient
    }
}

// 1
pub async fn dialogue_start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log::info!("dialogue_start function used");

    let client_chat_id  = dialogue.chat_id();

    bot.send_message(msg.chat.id, format!("Let's start! which user do u wanna stalk?\n\nCurrent chat id is {:?}", client_chat_id))
        .await?;
    dialogue.update(State::DialogueStalk { client_chat_id }).await?;
    Ok(())
}

// 2
pub async fn dialogue_stalk(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log::info!("dialogue_stalk function used");

    // retrieving state xdd [1]
    let current_state = dialogue.get().await?.unwrap();
    let client_chat_id = if let State::DialogueStalk { client_chat_id } = current_state {
        client_chat_id
    } else {
        return Ok(());
    };

    // checking if user input is a Recipient::ChannelUsername variant then proceeding w/button input
    if let Some(username) = msg.text() {
        if username.contains(' ') {
            bot.send_message(msg.chat.id, "username can't contain spaces")
                .await?;
            return Ok(());
        }
        let target_channel_username = Recipient::ChannelUsername(username.to_string());

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
    
        bot.send_message(msg.chat.id, format!("Okay, now select storage to save {:?}'s data to", target_channel_username))
            .reply_markup(keyboard)
            .await?;
    
        dialogue.update(State::DialogueSetStorage { client_chat_id, target_channel_username }).await?;
    } else {
        bot.send_message(msg.chat.id, "sry, invalid channel username, try again")
        .await?;
    };

    Ok(())
}

// 3, gets triggered on button click
pub async fn handle_storage_callback(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
) -> HandlerResult {
    log::info!("handle_storage_callback function used");

    // retrieving state xdd [2]
    let current_state = dialogue.get().await?.unwrap();
    let (client_chat_id, target_channel_username) = 
        if let State::DialogueSetStorage { client_chat_id, target_channel_username } = current_state {
            (client_chat_id, target_channel_username)
        } else {
            return Ok(());
        };


    if let Some(msg) = q.message {
        if let Some(data_str) = q.data {
            match serde_json::from_str::<StorageTypeCallback>(&data_str) {
                Ok(data) => {
                    bot.send_message(
                        msg.chat().id,
                        format!("u chose {:?} storage. saving your config.\n\ncurrent chat id: {:?}, stalking {:?}", data.storage_type, client_chat_id, target_channel_username),
                    )
                    .await?;

                }
                Err(err) => {
                    log::error!("Failed to parse StorageTypeCallback: {:?}", err);
                    bot.send_message(msg.chat().id, "Invalid selection.")
                        .await?;
                }
            }
        }
        Ok(())
    } else {
        log::error!("something went wrong w buttons (no message)");
        Ok(())
    }
}

// pub async fn save_config(client_chat_id: ChatId, target_channel_id, storage) {
    
// }