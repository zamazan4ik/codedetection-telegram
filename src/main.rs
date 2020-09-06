mod detection;
mod commands;

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

                    // Handle commands
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
                        static FORMAT_TEXT: &str = "Оберните код в теги: 3 символа ` до и после кода \
                        (в случае одиночной конструкции достаточно 1 ` с обеих сторон). Спасибо!";

                        let response_message = message
                            .reply_to(FORMAT_TEXT)
                            .send()
                            .await;

                        match response_message {
                            Ok(extracted_response_message) =>
                                {
                                    bot_responses_to_messages
                                        .lock()
                                        .unwrap()
                                        .insert(message.update.id, extracted_response_message.id);
                                }
                            Err(_) => { response_message.log_on_error().await; }
                        };
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

                        match maybe_bot_answer_id {
                            Some(response) => {
                                // Clear all related to the message bot responses
                                message.bot
                                    .delete_message(message.chat_id(), response)
                                    .send()
                                    .await
                                    .log_on_error()
                                    .await;

                                bot_responses_to_messages
                                    .lock()
                                    .unwrap()
                                    .remove(&message.update.id);

                                return;
                            }
                            None => { return; }
                        }
                    } else if detection::is_code_detected(message_text) {
                        // Delete old notification, then create new notification

                        let old_notification = bot_responses_to_messages
                            .lock()
                            .unwrap()
                            .get(&message.update.id)
                            .cloned();

                        match old_notification {
                            Some(old_id) => {
                                message.bot
                                    .delete_message(message.chat_id(), old_id)
                                    .send()
                                    .await
                                    .log_on_error()
                                    .await;

                                bot_responses_to_messages
                                    .lock()
                                    .unwrap()
                                    .remove(&message.update.id);
                            }
                            None => ()
                        }

                        static FORMAT_TEXT: &str = "Всё ещё неправильно :( Оберните код в теги: 3 символа ` до и после кода \
                        (в случае одиночной конструкции достаточно 1 ` с обеих сторон). Спасибо!";

                        let response_message = message
                            .reply_to(FORMAT_TEXT)
                            .send()
                            .await;

                        match response_message {
                            Ok(extracted_response_message) =>
                                {
                                    bot_responses_to_messages
                                        .lock()
                                        .unwrap()
                                        .insert(message.update.id, extracted_response_message.id);
                                }
                            Err(_) => { response_message.log_on_error().await; }
                        };
                    }
                }
            })
        })
        .dispatch()
        .await;
}
