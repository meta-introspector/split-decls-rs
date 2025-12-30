// Generated macro for remove_previous_ws_if_single_opening_tag_requires_it (function)
macro_rules! Depcrate_parser_tests_whitespaceremove_previous_ws_if_single_opening_tag_requires_it {
() => {
// Module: crate::parser::tests::whitespace
// Provides: {"remove_previous_ws_if_single_opening_tag_requires_it"}
// Dependencies: {}
# [test] fn remove_previous_ws_if_single_opening_tag_requires_it () { let ws = WS { left : true , right : false } ; let ast = vec ! [Node :: Text ("hey " . to_string ()) , Node :: ImportMacro (ws , "hey " . to_string () , "ho" . to_string ()) ,] ; assert_eq ! (remove_whitespace (ast , None) , vec ! [Node :: Text ("hey" . to_string ()) , Node :: ImportMacro (ws , "hey " . to_string () , "ho" . to_string ()) ,]) ; }
};
}
