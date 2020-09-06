mod commands;
mod detection;
mod utils;

use teloxide::{prelude::*, utils::command::BotCommand};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting CodeDetector bot!");

    let bot = Bot::from_env();

    let bot_responses_to_messages = Arc::new(Mutex::new(HashMap::<i32, i32>::new()));
    let bot_responses_to_edited_messages = bot_responses_to_messages.clone();


    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each(move |message| {
                let bot_responses_to_messages = bot_responses_to_messages.clone();
                async move {
                    let message_text = match message.update.text() {
                        Some(x) => x,
                        None => return,
                    };

                    // Handle commands. If command cannot be parsed - continue processing
                    match commands::Command::parse(message_text, "CodeDetectorBot") {
                        Ok(command) => {
                            commands::command_answer(&message, command).await.log_on_error().await;
                            return;
                        }
                        Err(_) => (),
                    };

                    // If message is formatted - just ignore it
                    if detection::maybe_formatted(message.update.entities()) {
                        return;
                    }

                    if detection::is_code_detected(message_text) {
                        utils::send_first_notification(&message, bot_responses_to_messages).await;
                    }
                }
            })
        })
        .edited_messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each(move |message| {
                let bot_responses_to_messages = bot_responses_to_edited_messages.clone();
                async move {
                    let message_text = match message.update.text() {
                        Some(x) => x,
                        None => return,
                    };

                    // Handle code formatting
                    if detection::maybe_formatted(message.update.entities()) {
                        let maybe_bot_answer_id =
                            bot_responses_to_messages
                                .lock()
                                .unwrap()
                                .get(&message.update.id)
                                .cloned();

                        if let Some(response) = maybe_bot_answer_id {
                            // Clear all related to the message bot responses
                            utils::delete_message(&message.bot, message.chat_id(),
                                                  response, bot_responses_to_messages.clone(),
                                                  &message.update.id).await;
                        }
                    } else if detection::is_code_detected(message_text) {
                        // Delete old notification, then create a new notification

                        let old_notification = bot_responses_to_messages
                            .lock()
                            .unwrap()
                            .get(&message.update.id)
                            .cloned();

                        if let Some(old_id) = old_notification {
                            utils::delete_message(&message.bot, message.chat_id(),
                                                  old_id, bot_responses_to_messages.clone(),
                                                  &message.update.id).await;
                        }

                        utils::send_another_notification(&message, bot_responses_to_messages).await;
                    }
                }
            })
        })
        .dispatch()
        .await;
}
