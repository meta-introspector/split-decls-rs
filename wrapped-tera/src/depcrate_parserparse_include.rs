// Generated macro for parse_include (function)
macro_rules! Depcrate_parserparse_include {
() => {
// Module: crate::parser
// Provides: {"parse_include"}
// Dependencies: {}
fn parse_include (pair : Pair < Rule >) -> Node { let mut ws = WS :: default () ; let mut files = vec ! [] ; let mut ignore_missing = false ; for p in pair . into_inner () { match p . as_rule () { Rule :: tag_start => { ws . left = p . as_span () . as_str () == "{%-" ; } Rule :: string => { files . push (replace_string_markers (p . as_span () . as_str ())) ; } Rule :: string_array => files . extend (parse_string_array (p)) , Rule :: ignore_missing => ignore_missing = true , Rule :: tag_end => { ws . right = p . as_span () . as_str () == "-%}" ; } _ => unreachable ! () , } ; } Node :: Include (ws , files , ignore_missing) }
};
}
