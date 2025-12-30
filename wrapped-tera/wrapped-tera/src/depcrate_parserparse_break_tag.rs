// Generated macro for parse_break_tag (function)
macro_rules! Depcrate_parserparse_break_tag {
() => {
// Module: crate::parser
// Provides: {"parse_break_tag"}
// Dependencies: {}
fn parse_break_tag (pair : Pair < Rule >) -> Node { let mut ws = WS :: default () ; for p in pair . into_inner () { match p . as_rule () { Rule :: tag_start => { ws . left = p . as_span () . as_str () == "{%-" ; } Rule :: tag_end => { ws . right = p . as_span () . as_str () == "-%}" ; } _ => unreachable ! () , } ; } Node :: Break (ws) }
};
}
