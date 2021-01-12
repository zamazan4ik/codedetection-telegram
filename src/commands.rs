use teloxide::{prelude::*, utils::command::BotCommand};

static HELP_TEXT: &str = "Для форматирования однострочной конструкции используйте\
        обрамление одиночным символом ` с обеих сторон. Для многострочной конструкции используйте\
        обрамление с помощью ``` до и после конструкции. Спасибо!";

static ABOUT_TEXT: &str = "По всем замечаниям или предложениям обращаться сюда:\
        https://github.com/ZaMaZaN4iK/codedetection-telegram . Спасибо!";

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "display info about bot.")]
    About,
}

pub async fn command_answer(cx: &UpdateWithCx<Message>, command: Command) -> ResponseResult<()> {

    match command {
        Command::Help => cx.reply_to(HELP_TEXT).send().await?,
        Command::About => cx.reply_to(ABOUT_TEXT).send().await?,
    };

    Ok(())
}
