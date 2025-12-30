// Generated macro for token (function)
macro_rules! Depcrate_parser_tokentoken {
() => {
// Module: crate::parser::token
// Provides: {"token"}
// Dependencies: {}
# [doc = " Creates a parser for the given [ParseToken]."] pub (super) fn token < 'a , P : ParseToken > (token : P) -> impl FnMut (Span < 'a >) -> ParseResult < P > { move | span | { let (span , _) = token . parse_token (span) ? ; Ok ((span , token)) } }
};
}
