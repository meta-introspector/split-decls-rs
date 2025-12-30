// Generated macro for dollar_cmd_sub (function)
macro_rules! Depcrate_parser_expansiondollar_cmd_sub {
() => {
// Module: crate::parser::expansion
// Provides: {"dollar_cmd_sub"}
// Dependencies: {}
fn dollar_cmd_sub (span : Span) -> ParseResult < Option < Term > > { delimited (pair (tag ("$(") , multi_trivia) , alt ((map (peek (alt ((char (')') , value (char :: default () , eof)))) , | _ | { None }) , map (term , Some) ,)) , context ("expected a closing )" , char (')')) ,) (span) }
};
}
