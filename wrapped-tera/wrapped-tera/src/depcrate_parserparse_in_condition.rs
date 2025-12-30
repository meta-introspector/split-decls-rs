// Generated macro for parse_in_condition (function)
macro_rules! Depcrate_parserparse_in_condition {
() => {
// Module: crate::parser
// Provides: {"parse_in_condition"}
// Dependencies: {}
fn parse_in_condition (pair : Pair < Rule >) -> TeraResult < Expr > { let mut lhs = None ; let mut rhs = None ; let mut negated = false ; for p in pair . into_inner () { match p . as_rule () { Rule :: string_expr_filter => lhs = Some (parse_string_expr_with_filters (p) ?) , Rule :: basic_expr_filter => lhs = Some (parse_basic_expr_with_filters (p) ?) , Rule :: in_cond_container => rhs = Some (parse_in_condition_container (p) ?) , Rule :: op_not => negated = true , _ => unreachable ! ("Got {:?} in parse_in_condition" , p) , } ; } Ok (Expr :: new (ExprVal :: In (In { lhs : Box :: new (lhs . unwrap ()) , rhs : Box :: new (rhs . unwrap ()) , negated , }))) }
};
}
