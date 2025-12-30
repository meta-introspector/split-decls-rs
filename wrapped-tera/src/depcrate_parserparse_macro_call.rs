// Generated macro for parse_macro_call (function)
macro_rules! Depcrate_parserparse_macro_call {
() => {
// Module: crate::parser
// Provides: {"parse_macro_call"}
// Dependencies: {}
fn parse_macro_call (pair : Pair < Rule >) -> TeraResult < MacroCall > { let mut namespace = None ; let mut name = None ; let mut args = HashMap :: new () ; for p in pair . into_inner () { match p . as_rule () { Rule :: ident => { if namespace . is_none () { namespace = Some (p . as_span () . as_str () . to_string ()) ; } else { name = Some (p . as_span () . as_str () . to_string ()) ; } } Rule :: kwarg => { let (key , val) = parse_kwarg (p) ? ; args . insert (key , val) ; } _ => unreachable ! ("Got {:?} in parse_macro_call" , p . as_rule ()) , } } Ok (MacroCall { namespace : namespace . unwrap () , name : name . unwrap () , args }) }
};
}
