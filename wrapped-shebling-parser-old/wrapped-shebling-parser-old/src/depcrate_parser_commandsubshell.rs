// Generated macro for subshell (function)
macro_rules! Depcrate_parser_commandsubshell {
() => {
// Module: crate::parser::command
// Provides: {"subshell"}
// Dependencies: {}
pub (super) fn subshell (span : Span) -> ParseResult < Term > { delimited (pair (char ('(') , multi_trivia) , term , tuple ((multi_trivia , context ("expected a closing )!" , char (')')) , trivia ,)) ,) (span) }
};
}
