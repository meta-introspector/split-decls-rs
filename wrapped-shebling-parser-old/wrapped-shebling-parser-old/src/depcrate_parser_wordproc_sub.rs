// Generated macro for proc_sub (function)
macro_rules! Depcrate_parser_wordproc_sub {
() => {
// Module: crate::parser::word
// Provides: {"proc_sub"}
// Dependencies: {}
fn proc_sub (span : Span) -> ParseResult < Vec < Term > > { delimited (tuple ((one_of ("<>") , char ('(') , multi_trivia)) , many0 (term) , pair (multi_trivia , char (')')) ,) (span) }
};
}
