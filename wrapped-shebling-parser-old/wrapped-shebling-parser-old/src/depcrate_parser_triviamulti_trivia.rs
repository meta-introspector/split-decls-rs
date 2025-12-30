// Generated macro for multi_trivia (function)
macro_rules! Depcrate_parser_triviamulti_trivia {
() => {
// Module: crate::parser::trivia
// Provides: {"multi_trivia"}
// Dependencies: {}
pub (super) fn multi_trivia (span : Span) -> ParseResult < String > { map (separated_list0 (line_ending , trivia) , | trivia | { trivia . join ("\n") }) (span) }
};
}
