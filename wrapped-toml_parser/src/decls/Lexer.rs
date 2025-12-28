macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! Lexer {
    () => {
        deps!();
        # [doc = " Lex TOML [tokens][Token]"] # [doc = ""] # [doc = " To get started, see [`Source::lex`][crate::Source::lex]"] pub struct Lexer < 'i > { stream : Stream < 'i > , eof : bool , }
    };
}

Lexer!()