use once_cell::sync::OnceCell;
use once_cell_regex::regex;
use regex::Regex;
use teloxide::{prelude::*, types::*, utils::command::BotCommand};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "display info about bot.")]
    About,
}

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
                match Command::parse(message_text, "CodeDetectorBot") {
                    Ok(command) => {
                        command_answer(&message, command).await.log_on_error().await;
                        return;
                    }
                    Err(_) => (),
                };

                // Handle code formatting
                if maybe_formatted(message.update.entities()) {
                    return;
                }

                if is_code_detected(message_text) {
                    message
                        .reply_to(
                            "Оберните код в теги: 3 символа ` до и после кода \
                        (в случае одиночной конструкции достаточно 1 ` с обеих сторон). Спасибо!",
                        )
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

async fn command_answer(cx: &UpdateWithCx<Message>, command: Command) -> ResponseResult<()> {
    static HELP_TEXT: &str = "Для форматирования однострочной конструкции используйте\
        обрамление одиночным символом ` с обеих сторон. Для многострочной конструкции используйте\
        обрамление с помощью ``` до и после конструкции. Спасибо!";

    static ABOUT_TEXT: &str = "По всем замечаниям или предложениям обращаться сюда:\
        https://github.com/ZaMaZaN4iK/codedetection-telegram . Спасибо!";

    match command {
        Command::Help => cx.reply_to(HELP_TEXT).send().await?,
        Command::About => cx.reply_to(ABOUT_TEXT).send().await?,
    };

    Ok(())
}

fn maybe_formatted(maybe_entities: Option<&[MessageEntity]>) -> bool {
    let entities = match maybe_entities {
        Some(entities) => entities,
        None => return false,
    };

    for entity in entities.iter() {
        match entity.kind {
            MessageEntityKind::Code | MessageEntityKind::Pre { .. } => return true,
            _ => (),
        }
    }

    return false;
}

fn is_code_detected(text: &str) -> bool {
    static INSTANCE: OnceCell<[&'static str; 76]> = OnceCell::new();
    let re : &Regex = regex!(INSTANCE.get_or_init(|| {
        [
            "namespace",
            "main",
            "cout",
            "cin",
            "printf",
            "scanf",
            "#include",
            "import",
            "while",
            "for",
            "async",
            "await",
            "yield",
            "concept",
            "alignas",
            "alignof",
            "asm",
            "atomic",
            "auto",
            "bitand",
            "bitor",
            "bool",
            "break",
            "case",
            "catch",
            "class",
            "compl",
            "const",
            "continue",
            "decltype",
            "declval",
            "default",
            "define",
            "delete",
            "new",
            "malloc",
            "free",
            "_cast",
            "if",
            "else",
            "enum",
            "explicit",
            "export",
            "extern",
            "friend",
            "goto",
            "mutable",
            "nullptr",
            "noexcept",
            "private",
            "protected",
            "public",
            "register",
            "requires",
            "return",
            "static",
            "assert",
            "struct",
            "switch",
            "template",
            "thread",
            "throw",
            "typedef",
            "using",
            "volatile",
            "typename",
            "union",
            "typeid",
            "virtual",
            "module",
            "final",
            "override",
            "float",
            "double",
            "void",
            "vector"
        ]
    }).join("|").as_str());

    // Just a random number, high enough
    const THRESHOLD: usize = 3;
    if re.find_iter(text).count() > THRESHOLD {
        return true;
    }

    return false;
}
