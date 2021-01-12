mod commands;
mod detection;
mod logging;
mod utils;
mod webhook;

use teloxide::{prelude::*, utils::command::BotCommand};

use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests {
    use crate::detection::is_code_detected;

    #[test]
    fn is_code() {
        assert!(is_code_detected("int main(){std::cout<<hello<<'\n'; return 0;}", 3));
    }
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    logging::init_logger();
    log::info!("Starting CodeDetector bot");

    let is_webhook_mode_enabled = env::var("WEBHOOK_MODE")
        .unwrap_or("false".to_string())
        .parse::<bool>()
        .expect(
            "Cannot convert WEBHOOK_MODE to bool. Applicable values are only \"true\" or \"false\"",
        );

    let threshold = env::var("THRESHOLD")
        .unwrap_or("3".to_string())
        .parse::<u8>()
        .expect("Cannot convert THRESHOLD to u8");

    let bot: Bot = Bot::from_env();

    let bot_responses_to_messages = Arc::new(Mutex::new(HashMap::<i32, i32>::new()));
    let bot_responses_to_edited_messages = bot_responses_to_messages.clone();

    let bot_dispatcher = Dispatcher::new(bot.clone())
        .messages_handler(move |rx: DispatcherHandlerRx<Message>| {
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
                            commands::command_answer(&message, command)
                                .await
                                .log_on_error()
                                .await;
                            ()
                        }
                        Err(_) => (),
                    };

                    // If message is formatted - just ignore it
                    if detection::maybe_formatted(message.update.entities()) {
                        return
                    }

                    if detection::is_code_detected(message_text, threshold) {
                        utils::send_first_notification(&message, bot_responses_to_messages).await;
                    }
                }
            })
        })
        .edited_messages_handler(move |rx: DispatcherHandlerRx<Message>| {
            rx.for_each(move |message| {
                let bot_responses_to_messages = bot_responses_to_edited_messages.clone();
                async move {
                    let message_text = match message.update.text() {
                        Some(x) => x,
                        None => return,
                    };

                    // Handle code formatting
                    if detection::maybe_formatted(message.update.entities()) {
                        let maybe_bot_answer_id = bot_responses_to_messages
                            .lock()
                            .unwrap()
                            .get(&message.update.id)
                            .cloned();

                        if let Some(response) = maybe_bot_answer_id {
                            // Clear all related to the message bot responses
                            utils::delete_message(
                                &message.bot,
                                message.chat_id(),
                                response,
                                bot_responses_to_messages.clone(),
                                &message.update.id,
                            )
                            .await;
                        }
                    } else if detection::is_code_detected(message_text, threshold) {
                        // Delete old notification, then create a new notification

                        let old_notification = bot_responses_to_messages
                            .lock()
                            .unwrap()
                            .get(&message.update.id)
                            .cloned();

                        if let Some(old_id) = old_notification {
                            utils::delete_message(
                                &message.bot,
                                message.chat_id(),
                                old_id,
                                bot_responses_to_messages.clone(),
                                &message.update.id,
                            )
                            .await;
                        }

                        utils::send_another_notification(&message, bot_responses_to_messages).await;
                    }
                }
            })
        });

    if is_webhook_mode_enabled {
        log::info!("Webhook mode activated");
        let rx = webhook::webhook(bot);
        bot_dispatcher
            .dispatch_with_listener(
                rx.await,
                LoggingErrorHandler::with_custom_text("An error from the update listener"),
            )
            .await;
        return
    }

    log::info!("Long polling mode activated");
    bot.delete_webhook()
            .send()
            .await
            .expect("Cannot delete a webhook");
    bot_dispatcher.dispatch()
        .await;

}
