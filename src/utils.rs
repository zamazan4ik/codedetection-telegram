use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::*;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub async fn send_first_notification(message: &UpdateWithCx<Message>,
                                     response_storage: Arc<Mutex<HashMap<i32, i32>>>) {
    static FORMAT_TEXT: &str = "Оберните код в теги: 3 символа ` до и после кода \
                        (в случае одиночной конструкции достаточно 1 ` с обеих сторон). Спасибо!";
    send_message(message, FORMAT_TEXT, response_storage).await;
}

pub async fn send_another_notification(message: &UpdateWithCx<Message>,
                                       response_storage: Arc<Mutex<HashMap<i32, i32>>>) {
    static FORMAT_TEXT: &str = "Всё ещё неправильно :( Оберните код в теги: 3 символа ` до и после кода \
                        (в случае одиночной конструкции достаточно 1 ` с обеих сторон). Спасибо!";
    send_message(message, FORMAT_TEXT, response_storage).await;
}

async fn send_message(message: &UpdateWithCx<Message>, text: &str,
                          response_storage: Arc<Mutex<HashMap<i32, i32>>>) {
    let response_message = message
        .reply_to(text)
        .send()
        .await;

    match response_message {
        Ok(extracted_response_message) =>
            {
                response_storage
                    .lock()
                    .unwrap()
                    .insert(message.update.id, extracted_response_message.id);
            }
        Err(_) => { response_message.log_on_error().await; }
    };
}

pub async fn delete_message<C>(bot: &Bot, chat_id: C, message_id: i32, response_storage: Arc<Mutex<HashMap<i32, i32>>>,
    original_id: &i32)
    where C: Into<ChatId>
{
    bot.delete_message(chat_id, message_id)
        .send()
        .await
        .log_on_error()
        .await;

    response_storage
        .lock()
        .unwrap()
        .remove(original_id);
}
