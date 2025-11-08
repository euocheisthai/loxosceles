use crate::dialogues::HandlerResult;
use std::{
    env,
};
use teloxide::{
    dispatching::UpdateHandler,
    dispatching::{dialogue, dialogue::InMemStorage},
    prelude::*,
    utils::command::BotCommands,
};
use teloxide_core::types::Message;
use tokio;

mod dialogues;
mod models;
use dialogues::{
    State,
    dialogue_stalk,
    dialogue_start,
    dialogue_storage_callback
};

mod loxosceles_mongo;
use loxosceles_mongo::{establish_mongo_connection, prepare_mongo};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Display help.")]
    Help,
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

// TODO; placeholder
async fn set_default_storage(bot: Bot, msg: Message, storage: String) -> HandlerResult {
    bot.send_message(msg.chat.id, format!("Default storage set to: {storage}"))
        .await?;
    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Unable to handle the message. Type /help to see command options. Or wallow in despair, because what you want is not implemented yet!",
    )
    .await?;
    Ok(())
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(display_help))
        // this one kickstarts the main dialogue and sets the state to DialogueStalk. State cases are handed in message_handler over there
        .branch(case![Command::Stalk].endpoint(dialogue_start))
        .branch(case![Command::Storage { storage }].endpoint(set_default_storage));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::DialogueStalk { client_chat_id, client_user_id }].endpoint(dialogue_stalk))
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler =
        Update::filter_callback_query().branch(dptree::endpoint(dialogue_storage_callback));

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

    let (_mongo_client, mongo_database) = establish_mongo_connection()
        .await
        .unwrap();

    // lets assume im returning N collections here and storing them in some "prepared_collections" var
    // where do i put them?
    // i imagine some Arc state structure? but wont the collections be mutated?
    // the docs read "Collection uses std::sync::Arc internally, so it can safely be shared across threads or async tasks."
    // so yeah that seems to be fine, still no idea where to store that..
    prepare_mongo(mongo_database).await;

    log::info!("Starting loxosceles bot...");
    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
