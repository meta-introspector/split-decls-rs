macro_rules! deps {
    () => {
        SyntaxTreeBuilder!();
        SyntaxError!();
    };
}

macro_rules! build_tree {
    () => {
        deps!();
        pub (crate) fn build_tree (lexed : parser :: LexedStr < '_ > , parser_output : parser :: Output ,) -> (GreenNode , Vec < SyntaxError > , bool) { let _p = tracing :: info_span ! ("build_tree") . entered () ; let mut builder = SyntaxTreeBuilder :: default () ; let is_eof = lexed . intersperse_trivia (& parser_output , & mut | step | match step { parser :: StrStep :: Token { kind , text } => builder . token (kind , text) , parser :: StrStep :: Enter { kind } => builder . start_node (kind) , parser :: StrStep :: Exit => builder . finish_node () , parser :: StrStep :: Error { msg , pos } => { builder . error (msg . to_owned () , pos . try_into () . unwrap ()) } }) ; let (node , mut errors) = builder . finish_raw () ; for (i , err) in lexed . errors () { let text_range = lexed . text_range (i) ; let text_range = TextRange :: new (text_range . start . try_into () . unwrap () , text_range . end . try_into () . unwrap () ,) ; errors . push (SyntaxError :: new (err , text_range)) } (node , errors , is_eof) }
    };
}

build_tree!()