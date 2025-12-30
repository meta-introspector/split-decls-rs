// Generated macro for bats_test (function)
macro_rules! Depcrate_parser_functionbats_test {
() => {
// Module: crate::parser::function
// Provides: {"bats_test"}
// Dependencies: {}
pub (super) fn bats_test (span : Span) -> ParseResult < Function > { let (span , mut header) = preceded (pair (tag ("@test ") , trivia) , peek (recognize_string (is_not ("\n"))) ,) (span) ? ; if let Some (header_len) = header . rfind (" {") { header . truncate (header_len) ; } else { return context ("invalid test name!" , fail) (span) ; } let (span , body) = preceded (pair (take (header . len ()) , trivia) , context ("invalid test body!" , brace_group) ,) (span) ? ; Ok ((span , Function :: new (header . trim_end () , body))) }
};
}
