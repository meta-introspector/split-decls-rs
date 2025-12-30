// Generated macro for parse_basic_expr_with_filters (function)
macro_rules! Depcrate_parserparse_basic_expr_with_filters {
() => {
// Module: crate::parser
// Provides: {"parse_basic_expr_with_filters"}
// Dependencies: {}
# [doc = " A basic expression with optional filters"] fn parse_basic_expr_with_filters (pair : Pair < Rule >) -> TeraResult < Expr > { let mut expr_val = None ; let mut filters = vec ! [] ; for p in pair . into_inner () { match p . as_rule () { Rule :: basic_expr => expr_val = Some (parse_basic_expression (p) ?) , Rule :: filter => filters . push (parse_filter (p) ?) , _ => unreachable ! ("Got {:?}" , p) , } ; } Ok (Expr { val : expr_val . unwrap () , negated : false , filters }) }
};
}
