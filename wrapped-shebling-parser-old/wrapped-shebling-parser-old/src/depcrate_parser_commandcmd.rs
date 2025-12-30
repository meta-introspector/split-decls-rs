// Generated macro for cmd (function)
macro_rules! Depcrate_parser_commandcmd {
() => {
// Module: crate::parser::command
// Provides: {"cmd"}
// Dependencies: {}
fn cmd (span : Span) -> ParseResult < Cmd > { alt ((into (compound_cmd) , into (cond_cmd) , into (coproc) , into (simple_cmd) ,)) (span) }
};
}
