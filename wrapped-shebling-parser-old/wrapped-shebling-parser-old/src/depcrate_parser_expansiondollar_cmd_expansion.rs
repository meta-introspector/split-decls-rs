// Generated macro for dollar_cmd_expansion (function)
macro_rules! Depcrate_parser_expansiondollar_cmd_expansion {
() => {
// Module: crate::parser::expansion
// Provides: {"dollar_cmd_expansion"}
// Dependencies: {}
fn dollar_cmd_expansion (span : Span) -> ParseResult < Term > { delimited (tuple ((tag ("${") , whitespace , multi_trivia)) , term , context ("expected a closing }" , char ('}')) ,) (span) }
};
}
