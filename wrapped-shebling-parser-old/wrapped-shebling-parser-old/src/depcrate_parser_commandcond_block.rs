// Generated macro for cond_block (function)
macro_rules! Depcrate_parser_commandcond_block {
() => {
// Module: crate::parser::command
// Provides: {"cond_block"}
// Dependencies: {}
fn cond_block < 'a > (keyword : Keyword) -> impl FnMut (Span < 'a >) -> ParseResult < CondBlock > { preceded (pair (token (keyword) , trivia) , map (pair (term , do_group) , | (cond , block) | { CondBlock :: new (cond , block) }) ,) }
};
}
