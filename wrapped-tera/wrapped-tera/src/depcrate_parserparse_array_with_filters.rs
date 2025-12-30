// Generated macro for parse_array_with_filters (function)
macro_rules! Depcrate_parserparse_array_with_filters {
() => {
// Module: crate::parser
// Provides: {"parse_array_with_filters"}
// Dependencies: {}
# [doc = " An array with optional filters"] fn parse_array_with_filters (pair : Pair < Rule >) -> TeraResult < Expr > { let mut array = None ; let mut filters = vec ! [] ; for p in pair . into_inner () { match p . as_rule () { Rule :: array => array = Some (parse_array (p) ?) , Rule :: filter => filters . push (parse_filter (p) ?) , _ => unreachable ! ("Got {:?}" , p) , } ; } Ok (Expr { val : array . unwrap () , negated : false , filters }) }
};
}
