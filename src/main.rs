mod detection;
mod commands;

use teloxide::{prelude::*, utils::command::BotCommand};

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting CodeDetector bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each(|message| async move {
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

                // Handle code formatting
                if detection::maybe_formatted(message.update.entities()) {
                    return;
                }

                if detection::is_code_detected(message_text) {
                    static FORMAT_TEXT: &str = "Оберните код в теги: 3 символа ` до и после кода \
                        (в случае одиночной конструкции достаточно 1 ` с обеих сторон). Спасибо!";

                    message
                        .reply_to(FORMAT_TEXT)
                        .send()
                        .await
                        .log_on_error()
                        .await;
                }
            })
        })
        .dispatch()
        .await;
}
