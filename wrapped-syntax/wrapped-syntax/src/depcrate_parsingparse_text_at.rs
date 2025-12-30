// Generated macro for parse_text_at (function)
macro_rules! Depcrate_parsingparse_text_at {
() => {
// Module: crate::parsing
// Provides: {"parse_text_at"}
// Dependencies: {}
pub (crate) fn parse_text_at (text : & str , entry : parser :: TopEntryPoint , edition : parser :: Edition ,) -> (GreenNode , Vec < SyntaxError >) { let _p = tracing :: info_span ! ("parse_text_at") . entered () ; let lexed = parser :: LexedStr :: new (edition , text) ; let parser_input = lexed . to_input (edition) ; let parser_output = entry . parse (& parser_input , edition) ; let (node , errors , _eof) = build_tree (lexed , parser_output) ; (node , errors) }
};
}
