// Generated macro for parse_string_concat (function)
macro_rules! Depcrate_parserparse_string_concat {
() => {
// Module: crate::parser
// Provides: {"parse_string_concat"}
// Dependencies: {}
fn parse_string_concat (pair : Pair < Rule >) -> TeraResult < ExprVal > { let mut values = vec ! [] ; let mut current_str = String :: new () ; for p in pair . into_inner () { match p . as_rule () { Rule :: string => { current_str . push_str (& replace_string_markers (p . as_str ())) ; } Rule :: int => { if ! current_str . is_empty () { values . push (ExprVal :: String (current_str)) ; current_str = String :: new () ; } values . push (ExprVal :: Int (p . as_str () . parse () . map_err (| _ | { Error :: msg (format ! ("Integer out of bounds: `{}`" , p . as_str ())) }) ?)) ; } Rule :: float => { if ! current_str . is_empty () { values . push (ExprVal :: String (current_str)) ; current_str = String :: new () ; } values . push (ExprVal :: Float (p . as_str () . parse () . map_err (| _ | { Error :: msg (format ! ("Float out of bounds: `{}`" , p . as_str ())) }) ? ,)) ; } Rule :: dotted_square_bracket_ident => { if ! current_str . is_empty () { values . push (ExprVal :: String (current_str)) ; current_str = String :: new () ; } values . push (ExprVal :: Ident (p . as_str () . to_string ())) } Rule :: fn_call => { if ! current_str . is_empty () { values . push (ExprVal :: String (current_str)) ; current_str = String :: new () ; } values . push (ExprVal :: FunctionCall (parse_fn_call (p) ?)) } _ => unreachable ! ("Got {:?} in parse_string_concat" , p) , } ; } if values . is_empty () { return Ok (ExprVal :: String (current_str)) ; } if ! current_str . is_empty () { values . push (ExprVal :: String (current_str)) ; } Ok (ExprVal :: StringConcat (StringConcat { values })) }
};
}
