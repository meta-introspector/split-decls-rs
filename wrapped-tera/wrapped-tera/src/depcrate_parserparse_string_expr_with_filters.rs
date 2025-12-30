// Generated macro for parse_string_expr_with_filters (function)
macro_rules! Depcrate_parserparse_string_expr_with_filters {
() => {
// Module: crate::parser
// Provides: {"parse_string_expr_with_filters"}
// Dependencies: {}
# [doc = " A string expression with optional filters"] fn parse_string_expr_with_filters (pair : Pair < Rule >) -> TeraResult < Expr > { let mut expr_val = None ; let mut filters = vec ! [] ; for p in pair . into_inner () { match p . as_rule () { Rule :: string => expr_val = Some (ExprVal :: String (replace_string_markers (p . as_str ()))) , Rule :: string_concat => expr_val = Some (parse_string_concat (p) ?) , Rule :: filter => filters . push (parse_filter (p) ?) , _ => unreachable ! ("Got {:?}" , p) , } ; } Ok (Expr { val : expr_val . unwrap () , negated : false , filters }) }
};
}
