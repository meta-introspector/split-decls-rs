// Generated macro for parse_set_tag (function)
macro_rules! Depcrate_parserparse_set_tag {
() => {
// Module: crate::parser
// Provides: {"parse_set_tag"}
// Dependencies: {}
fn parse_set_tag (pair : Pair < Rule > , global : bool) -> TeraResult < Node > { let mut ws = WS :: default () ; let mut key = None ; let mut expr = None ; for p in pair . into_inner () { match p . as_rule () { Rule :: tag_start => { ws . left = p . as_span () . as_str () == "{%-" ; } Rule :: tag_end => { ws . right = p . as_span () . as_str () == "-%}" ; } Rule :: ident => key = Some (p . as_str () . to_string ()) , Rule :: logic_expr => expr = Some (parse_logic_expr (p) ?) , Rule :: array_filter => expr = Some (parse_array_with_filters (p) ?) , _ => unreachable ! ("unexpected {:?} rule in parse_set_tag" , p . as_rule ()) , } } Ok (Node :: Set (ws , Set { key : key . unwrap () , value : expr . unwrap () , global })) }
};
}
