use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(Clone, BotCommands)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "display info about bot.")]
    About,
}

pub async fn command_handler(
    msg: Message,
    bot: AutoSend<Bot>,
    command: Command,
) -> anyhow::Result<()> {
    static HELP_TEXT: &str = "Для форматирования однострочной конструкции используйте\
        обрамление одиночным символом ` с обеих сторон. Для многострочной конструкции используйте\
        обрамление с помощью ``` до и после конструкции. Спасибо!";

    static ABOUT_TEXT: &str = "По всем замечаниям или предложениям обращаться сюда:\
        https://github.com/ZaMaZaN4iK/codedetection-telegram . Спасибо!";

    match command {
        Command::Help => {
            bot.send_message(msg.chat.id, HELP_TEXT)
                .reply_to_message_id(msg.id)
                .await?
        }
        Command::About => {
            bot.send_message(msg.chat.id, ABOUT_TEXT)
                .reply_to_message_id(msg.id)
                .await?
        }
    };

    Ok(())
}
