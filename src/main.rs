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
mod dialogues;
use dialogues::{
    State, dialogue_stalk, dialogue_start, handle_storage_callback,
};

mod test_input;
use test_input::{init_rc, test_input};

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

// placeholder
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
        .branch(case![State::DialogueStalk].endpoint(dialogue_stalk))
        //.branch(case![State::DialogueSetStorage { storage_type }].endpoint(dialogue_set_storage))
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler =
        Update::filter_callback_query().branch(dptree::endpoint(handle_storage_callback));      // right now missing a state in which this branch is triggered

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
