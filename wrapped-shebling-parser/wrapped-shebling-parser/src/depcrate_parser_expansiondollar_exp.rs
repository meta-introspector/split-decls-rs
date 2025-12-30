// Generated macro for dollar_exp (function)
macro_rules! Depcrate_parser_expansiondollar_exp {
() => {
// Module: crate::parser::expansion
// Provides: {"dollar_exp"}
// Dependencies: {}
pub (super) fn dollar_exp (span : ParseSpan) -> ParseResult < DollarExp > { preceded (peek (char ('$')) , alt ((delimited (tag ("$((") , into (arith_seq) , pair (char (')') , cut (context ("expected a double )) to end the $((..))." , char (')') ,)) ,) ,) , delimited (pair (char ('$') , char ('[')) , into (arith_seq) , char (']')) , into (param_expansion) , map (preceded (char ('$') , spanned (alt ((recognize_string (satisfy (| c | c . is_ascii_digit ())) , recognize_string (one_of (SPECIAL_PARAMS)) , identifier ,))) ,) , DollarExp :: Var ,) ,)) ,) (span) }
};
}
