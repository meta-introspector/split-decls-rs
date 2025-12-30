// Generated macro for subscript (function)
macro_rules! Depcrate_parser_wordsubscript {
() => {
// Module: crate::parser::word
// Provides: {"subscript"}
// Dependencies: {}
fn subscript (span : Span) -> ParseResult < String > { delimited (char ('[') , recognize_string (context ("empty subscript!" , many1 (alt ((word_sgmt_before_pattern ("]") , map (trivia1 , WordSgmt :: Lit) , map (recognize_string (is_a (& * format ! ("|&;<>() '\t\n\r\u{A0}{}" , DOUBLE_ESCAPABLE))) , WordSgmt :: Lit ,) ,))) ,)) , char (']') ,) (span) }
};
}
