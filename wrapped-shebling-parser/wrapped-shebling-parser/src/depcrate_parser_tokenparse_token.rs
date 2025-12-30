// Generated macro for parse_token (function)
macro_rules! Depcrate_parser_tokenparse_token {
() => {
// Module: crate::parser::token
// Provides: {"parse_token"}
// Dependencies: {}
# [doc = " Creates a parser for the given token based on its `token()` pattern."] fn parse_token < 'a , T : Token > (token : T) -> impl FnMut (ParseSpan < 'a >) -> ParseResult < T > { value (token , tag (token . token ())) }
};
}
