use crate::models::{StorageType, StorageTypeCallback};
use serde::{Deserialize, Serialize};
use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::Dialogue,
    prelude::*,
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message},
};

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

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
pub async fn dialogue_start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log::info!("dialogue_start function used");

    bot.send_message(msg.chat.id, "Let's start! which user do u wanna stalk?")
        .await?;
    dialogue.update(State::DialogueStalk).await?;
    Ok(())
}

// 2
pub async fn dialogue_stalk(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
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
pub async fn handle_storage_callback(
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

        dialogue
            .update(State::DialogueSetStorage {
                storage_type: data.storage_type,
            })
            .await?;
        Ok(())
    } else {
        log::error!("something went wrong w buttons");
        Ok(())
    }
}

// 3.2
pub async fn dialogue_set_storage(
    bot: Bot,
    _dialogue: MyDialogue,
    msg: Message,
    storage_type: StorageType,
) -> HandlerResult {
    log::info!(
        "dialogue_set_storage function used and {:#?} set as storage",
        storage_type
    );

    match msg.text().map(|text| text.parse::<u8>()) {
        Some(Ok(_storage)) => {
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
