// Generated macro for remove_next_ws_if_single_opening_tag_requires_it (function)
macro_rules! Depcrate_parser_tests_whitespaceremove_next_ws_if_single_opening_tag_requires_it {
() => {
// Module: crate::parser::tests::whitespace
// Provides: {"remove_next_ws_if_single_opening_tag_requires_it"}
// Dependencies: {}
# [test] fn remove_next_ws_if_single_opening_tag_requires_it () { let ws = WS { left : true , right : true } ; let ast = vec ! [Node :: ImportMacro (ws , "hey " . to_string () , "ho" . to_string ()) , Node :: Text ("  hey" . to_string ()) ,] ; assert_eq ! (remove_whitespace (ast , None) , vec ! [Node :: ImportMacro (ws , "hey " . to_string () , "ho" . to_string ()) , Node :: Text ("hey" . to_string ()) ,]) ; }
};
}
