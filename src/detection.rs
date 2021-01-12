use once_cell::sync::OnceCell;
use once_cell_regex::regex;
use regex::Regex;
use teloxide::types::*;


static INSTANCE: OnceCell<[&'static str; 76]> = OnceCell::new();

pub fn maybe_formatted(maybe_entities: Option<&[MessageEntity]>) -> bool {
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
    false
}

pub fn is_code_detected(text: &str, threshold: u8) -> bool {
    static INSTANCE: OnceCell<[&'static str; 76]> = OnceCell::new();
    let re: &Regex = regex!(INSTANCE
        .get_or_init(|| {
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
                "vector",
            ]
        })
        .join("|")
        .as_str());
    re.find_iter(text).count() > threshold as usize
}
