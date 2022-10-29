use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Грешка" => "Err",
        "Добре" => "Ok",
        "Конец" => "String",
        "ХашКарта" => "HashMap",
        "Стандарт" => "Default",
        "Неизправност" => "Error",
        "Опция" => "Option",
        "Някоя" => "Some",
        "Нищо" => "None",
        "Резултат" => "Result",
        "СебеСи" => "Self",
        "колекция" => "collections",
        "принтлиния" => "println",
        "счупи" => "break",
        "асинхронна" => "async",
        "изчакай" => "await",
        "цикъл" => "loop",
        "премести" => "move",
        "щайга" => "crate",
        "Кутия" => "Box",
        "недостижим_код" => "unreachable_code",
        "като" => "as",
        "константа" => "const",
        "черта" => "trait",
        "тип" => "type",
        "опасен" => "unsafe",
        "в" => "in",
        "от" => "from",
        "динамичен" => "dyn",
        "разопаковай" => "unwrap",
        "стандарт" => "default",
        "като_връзка" => "as_ref",
        // вход-изход
        "ви" => "io",
        "външна" => "extern",
        "лъжа" => "false",
        "функция" => "fn",
        "супер" => "super",
        "вмъкни" => "insert",

        // iterator funktionen
        "отново" => "iter",
        "в_отново" => "into_iter",
        "карта" => "map",
        "плоска_карта" => "flat_map",
        "сгъни" => "fold",
        "източи" => "drain",
        "събери" => "collect",
        "намери" => "find",
        "вземи" => "take", 
        "продукт" => "product",

        // ordering

        // сравни - compare
        "срв" => "cmp",
        "Ред" => "Ordering",
        "ПоГолямо" => "Greater",
        "ПоМалко" => "Less",
        "Равно" => "Equal",
        "вземи" => "get",
        "позволи" => "allow",
        "паника" | "фатална_грешка" => "panic",
        "модул" => "mod",
        "мутираща" => "mut",
        "нов" => "new",
        "къде" => "where",
        "за" => "for",
        "вземи_или_вмъкни_с" => "get_or_insert_with",
        "главна" => "main",
        "публична" => "pub",
        "не" => None?,
        "върни" => "return",
        "допълни" => "impl",
        "връзка" => "ref",
        "съвпадни" => "match",
        "ако" => "if",
        "иначе" => "else",
        "себеси" => "self",
        "нека" => "let",
        "статична" => "static",
        "структура" => "struct",
        "очаквам" => "expect",
        "докато" => "while",
        "използвай" => "use",
        "във" => "into",
        "" => "true",
        "изброяване" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn ruzhda(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
