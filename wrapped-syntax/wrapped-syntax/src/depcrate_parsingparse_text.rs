// Generated macro for parse_text (function)
macro_rules! Depcrate_parsingparse_text {
() => {
// Module: crate::parsing
// Provides: {"parse_text"}
// Dependencies: {}
pub (crate) fn parse_text (text : & str , edition : parser :: Edition) -> (GreenNode , Vec < SyntaxError >) { let _p = tracing :: info_span ! ("parse_text") . entered () ; let lexed = parser :: LexedStr :: new (edition , text) ; let parser_input = lexed . to_input (edition) ; let parser_output = parser :: TopEntryPoint :: SourceFile . parse (& parser_input , edition) ; let (node , errors , _eof) = build_tree (lexed , parser_output) ; (node , errors) }
};
}
