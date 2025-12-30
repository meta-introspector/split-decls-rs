// Generated macro for handle_ws_both_sides_for_raw_tag (function)
macro_rules! Depcrate_parser_tests_whitespacehandle_ws_both_sides_for_raw_tag {
() => {
// Module: crate::parser::tests::whitespace
// Provides: {"handle_ws_both_sides_for_raw_tag"}
// Dependencies: {}
# [test] fn handle_ws_both_sides_for_raw_tag () { let start_ws = WS { left : true , right : false } ; let end_ws = WS { left : true , right : true } ; let ast = vec ! [Node :: Raw (start_ws , "  hey " . to_string () , end_ws) , Node :: Text ("  hey" . to_string ())] ; assert_eq ! (remove_whitespace (ast , None) , vec ! [Node :: Raw (start_ws , "  hey" . to_string () , end_ws) , Node :: Text ("hey" . to_string ()) ,]) ; }
};
}
