use teloxide::prelude::*;
use teloxide::types::Recipient;
use teloxide::Bot;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub async fn send_first_notification(
    msg: Message,
    bot: AutoSend<Bot>,
    response_storage: Arc<Mutex<HashMap<i32, i32>>>,
) {
    static FORMAT_TEXT: &str = "Оберните код в теги: 3 символа ` до и после кода \
                        (в случае одиночной конструкции достаточно 1 ` с обеих сторон). Спасибо!";
    send_message(msg, bot, FORMAT_TEXT, response_storage).await;
}

pub async fn send_another_notification(
    msg: Message,
    bot: AutoSend<Bot>,
    response_storage: Arc<Mutex<HashMap<i32, i32>>>,
) {
    static ERROR_FORMAT_TEXT: &str =
        "Всё ещё неправильно :( Оберните код в теги: 3 символа ` до и после кода \
                        (в случае одиночной конструкции достаточно 1 ` с обеих сторон). Спасибо!";
    send_message(msg, bot, ERROR_FORMAT_TEXT, response_storage).await;
}

async fn send_message(
    msg: Message,
    bot: AutoSend<Bot>,
    text: &str,
    response_storage: Arc<Mutex<HashMap<i32, i32>>>,
) {
    let response_message = bot
        .send_message(msg.chat.id, text)
        .reply_to_message_id(msg.id)
        .await;

    match response_message {
        Ok(extracted_response_message) => {
            response_storage
                .lock()
                .unwrap()
                .insert(msg.id, extracted_response_message.id);
        }
        Err(_) => {
            response_message.log_on_error().await;
        }
    };
}

pub async fn delete_message<C>(
    bot: &AutoSend<Bot>,
    chat_id: C,
    message_id: i32,
    response_storage: Arc<Mutex<HashMap<i32, i32>>>,
    original_id: &i32,
) where
    C: Into<ChatId>,
    Recipient: std::convert::From<C>,
{
    bot.delete_message(chat_id, message_id)
        .await
        .log_on_error()
        .await;

    response_storage.lock().unwrap().remove(original_id);
}
