// Generated macro for dollar_exp (function)
macro_rules! Depcrate_parser_expansiondollar_exp {
() => {
// Module: crate::parser::expansion
// Provides: {"dollar_exp"}
// Dependencies: {}
pub (super) fn dollar_exp (span : Span) -> ParseResult < DollarExp > { preceded (peek (char ('$')) , alt ((delimited (tag ("$((") , into (arith_seq) , pair (char (')') , cut (context ("expected a double )) to end the $((..))." , char (')') ,)) ,) ,) , into (dollar_cmd_sub) , delimited (pair (char ('$') , char ('[')) , into (arith_seq) , char (']')) , into (dollar_cmd_expansion) , into (param_expansion) , map (dollar_var , DollarExp :: Var) ,)) ,) (span) }
};
}
