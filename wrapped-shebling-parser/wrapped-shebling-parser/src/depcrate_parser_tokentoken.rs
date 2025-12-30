// Generated macro for token (function)
macro_rules! Depcrate_parser_tokentoken {
() => {
// Module: crate::parser::token
// Provides: {"token"}
// Dependencies: {}
# [doc = " Returns a parser for the given [ParseToken]."] pub (super) fn token < 'a , P : ParseToken > (token : P) -> impl FnMut (ParseSpan < 'a >) -> ParseResult < P > { move | span | { let (span , _) = token . parse_token (span) ? ; Ok ((span , token)) } }
};
}
