mod commands;
mod detection;
mod logging;
mod utils;
mod webhook;

use teloxide::prelude::*;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests {
    use crate::detection::is_code_detected;

    #[test]
    fn is_code() {
        assert!(is_code_detected(
            "int main(){int hello = 3; cout<<hello<<'\n'; return 0;}",
            2
        ));
    }
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    logging::init_logger();
    log::info!("Starting CodeDetector bot");

    let is_webhook_mode_enabled = std::env::var("WEBHOOK_MODE")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .expect(
            "Cannot convert WEBHOOK_MODE to bool. Applicable values are only \"true\" or \"false\"",
        );

    let threshold = std::env::var("THRESHOLD")
        .unwrap_or_else(|_| "3".to_string())
        .parse::<u8>()
        .expect("Cannot convert THRESHOLD to u8");

    let bot = Bot::from_env().auto_send();

    let bot_responses_to_messages = Arc::new(Mutex::new(HashMap::<i32, i32>::new()));

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .branch(
                    dptree::entry()
                        .filter_command::<commands::Command>()
                        .endpoint(commands::command_handler),
                )
                .branch(
                    dptree::filter(|msg: Message| msg.text().is_some()).endpoint(
                        |msg: Message,
                         bot: AutoSend<Bot>,
                         bot_responses_to_messages: std::sync::Arc<
                            std::sync::Mutex<HashMap<i32, i32>>,
                        >,
                         threshold: u8| async move {
                            process_message(msg, bot, bot_responses_to_messages, threshold).await?;
                            anyhow::Result::Ok(())
                        },
                    ),
                ),
        )
        .branch(Update::filter_edited_message().endpoint(process_edited_message));

    if !is_webhook_mode_enabled {
        log::info!("Webhook deleted");
        bot.delete_webhook().await.expect("Cannot delete a webhook");
    }

    let mut bot_dispatcher = Dispatcher::builder(bot.clone(), handler)
        .dependencies(dptree::deps![bot_responses_to_messages.clone(), threshold])
        .default_handler(|_| async move {})
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build();

    if is_webhook_mode_enabled {
        log::info!("Webhook mode activated");
        let rx = webhook::webhook(bot);
        bot_dispatcher
            .dispatch_with_listener(
                rx.await,
                LoggingErrorHandler::with_custom_text("An error from the update listener"),
            )
            .await;
    } else {
        log::info!("Long polling mode activated");
        bot_dispatcher.dispatch().await;
    }
}

async fn process_message(
    msg: Message,
    bot: AutoSend<Bot>,
    bot_responses_to_messages: std::sync::Arc<std::sync::Mutex<HashMap<i32, i32>>>,
    threshold: u8,
) -> anyhow::Result<()> {
    let bot_responses_to_messages = bot_responses_to_messages.clone();
    // If message is formatted - just ignore it
    if detection::maybe_formatted(msg.entities()) {
        return Ok(());
    }

    if let Some(text) = msg.text() {
        if detection::is_code_detected(text, threshold) {
            utils::send_first_notification(msg, bot, bot_responses_to_messages).await;
        }
    }

    Ok(())
}

async fn process_edited_message(
    msg: Message,
    bot: AutoSend<Bot>,
    bot_responses_to_messages: std::sync::Arc<std::sync::Mutex<HashMap<i32, i32>>>,
    threshold: u8,
) -> anyhow::Result<()> {
    let message_text = match msg.text() {
        Some(x) => x,
        None => return Ok(()),
    };

    // Handle code formatting
    if detection::maybe_formatted(msg.entities()) {
        let maybe_bot_answer_id = bot_responses_to_messages
            .lock()
            .unwrap()
            .get(&msg.id)
            .cloned();

        if let Some(response) = maybe_bot_answer_id {
            // Clear all related to the message bot responses
            utils::delete_message(
                &bot,
                msg.chat.id,
                response,
                bot_responses_to_messages.clone(),
                &msg.id,
            )
            .await;
        }
    } else if detection::is_code_detected(message_text, threshold) {
        // Delete an old notification, then create a new notification

        let old_notification = bot_responses_to_messages
            .lock()
            .unwrap()
            .get(&msg.id)
            .cloned();

        if let Some(old_id) = old_notification {
            utils::delete_message(
                &bot,
                msg.chat.id,
                old_id,
                bot_responses_to_messages.clone(),
                &msg.id,
            )
            .await;
        }

        utils::send_another_notification(msg, bot, bot_responses_to_messages).await;
    }

    Ok(())
}
