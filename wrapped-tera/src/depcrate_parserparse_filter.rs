// Generated macro for parse_filter (function)
macro_rules! Depcrate_parserparse_filter {
() => {
// Module: crate::parser
// Provides: {"parse_filter"}
// Dependencies: {}
fn parse_filter (pair : Pair < Rule >) -> TeraResult < FunctionCall > { let mut name = None ; let mut args = HashMap :: new () ; for p in pair . into_inner () { match p . as_rule () { Rule :: ident => name = Some (p . as_span () . as_str () . to_string ()) , Rule :: kwarg => { let (name , val) = parse_kwarg (p) ? ; args . insert (name , val) ; } Rule :: fn_call => { return parse_fn_call (p) ; } _ => unreachable ! ("{:?} not supposed to get there (parse_filter)!" , p . as_rule ()) , } ; } Ok (FunctionCall { name : name . unwrap () , args }) }
};
}
