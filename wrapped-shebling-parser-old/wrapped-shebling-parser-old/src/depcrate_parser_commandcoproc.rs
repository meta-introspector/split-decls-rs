// Generated macro for coproc (function)
macro_rules! Depcrate_parser_commandcoproc {
() => {
// Module: crate::parser::command
// Provides: {"coproc"}
// Dependencies: {}
fn coproc (span : Span) -> ParseResult < Coproc > { preceded (pair (token (Keyword :: Coproc) , whitespace) , alt ((map (pair (opt (terminated (identifier , whitespace)) , compound_cmd) , | (name , cmd) | Coproc :: new (name , cmd) ,) , map (simple_cmd , | cmd | Coproc :: new (None , cmd)) ,)) ,) (span) }
};
}
