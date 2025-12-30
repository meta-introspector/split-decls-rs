// Generated macro for parse_import_macro (function)
macro_rules! Depcrate_parserparse_import_macro {
() => {
// Module: crate::parser
// Provides: {"parse_import_macro"}
// Dependencies: {}
fn parse_import_macro (pair : Pair < Rule >) -> Node { let mut ws = WS :: default () ; let mut file = None ; let mut ident = None ; for p in pair . into_inner () { match p . as_rule () { Rule :: tag_start => { ws . left = p . as_span () . as_str () == "{%-" ; } Rule :: string => file = Some (replace_string_markers (p . as_span () . as_str ())) , Rule :: ident => ident = Some (p . as_span () . as_str () . to_string ()) , Rule :: tag_end => { ws . right = p . as_span () . as_str () == "-%}" ; } _ => unreachable ! () , } ; } Node :: ImportMacro (ws , file . unwrap () , ident . unwrap ()) }
};
}
