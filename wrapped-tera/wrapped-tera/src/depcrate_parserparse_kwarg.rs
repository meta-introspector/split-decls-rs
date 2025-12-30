// Generated macro for parse_kwarg (function)
macro_rules! Depcrate_parserparse_kwarg {
() => {
// Module: crate::parser
// Provides: {"parse_kwarg"}
// Dependencies: {}
fn parse_kwarg (pair : Pair < Rule >) -> TeraResult < (String , Expr) > { let mut name = None ; let mut val = None ; for p in pair . into_inner () { match p . as_rule () { Rule :: ident => name = Some (p . as_span () . as_str () . to_string ()) , Rule :: logic_expr => val = Some (parse_logic_expr (p) ?) , Rule :: array_filter => val = Some (parse_array_with_filters (p) ?) , _ => unreachable ! ("{:?} not supposed to get there (parse_kwarg)!" , p . as_rule ()) , } ; } Ok ((name . unwrap () , val . unwrap ())) }
};
}
