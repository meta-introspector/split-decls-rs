// Generated macro for parse_content (function)
macro_rules! Depcrate_parserparse_content {
() => {
// Module: crate::parser
// Provides: {"parse_content"}
// Dependencies: {}
fn parse_content (pair : Pair < Rule >) -> TeraResult < Vec < Node > > { let pairs = pair . into_inner () ; let mut nodes = Vec :: with_capacity (pairs . len ()) ; for p in pairs { match p . as_rule () { Rule :: include_tag => nodes . push (parse_include (p)) , Rule :: comment_tag => nodes . push (parse_comment_tag (p)) , Rule :: super_tag => nodes . push (Node :: Super) , Rule :: set_tag => nodes . push (parse_set_tag (p , false) ?) , Rule :: set_global_tag => nodes . push (parse_set_tag (p , true) ?) , Rule :: raw => nodes . push (parse_raw_tag (p)) , Rule :: variable_tag => nodes . push (parse_variable_tag (p) ?) , Rule :: forloop => nodes . push (parse_forloop (p) ?) , Rule :: break_tag => nodes . push (parse_break_tag (p)) , Rule :: continue_tag => nodes . push (parse_continue_tag (p)) , Rule :: content_if | Rule :: macro_if | Rule :: block_if | Rule :: for_if | Rule :: filter_section_if => nodes . push (parse_if (p) ?) , Rule :: filter_section => nodes . push (parse_filter_section (p) ?) , Rule :: text => nodes . push (Node :: Text (p . as_span () . as_str () . to_string ())) , Rule :: block => nodes . push (parse_block (p) ?) , _ => unreachable ! ("unreachable content rule: {:?}" , p . as_rule ()) , } ; } Ok (nodes) }
};
}
