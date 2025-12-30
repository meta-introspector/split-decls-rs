// Generated macro for identifier (function)
macro_rules! Depcrate_parser_wordidentifier {
() => {
// Module: crate::parser::word
// Provides: {"identifier"}
// Dependencies: {}
pub (super) fn identifier (span : ParseSpan) -> ParseResult < String > { recognize_string (pair (alt ((alpha1 , tag ("_"))) , many0 (alt ((alphanumeric1 , tag ("_")))) ,)) (span) }
};
}
