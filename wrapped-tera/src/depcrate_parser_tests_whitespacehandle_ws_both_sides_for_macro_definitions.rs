// Generated macro for handle_ws_both_sides_for_macro_definitions (function)
macro_rules! Depcrate_parser_tests_whitespacehandle_ws_both_sides_for_macro_definitions {
() => {
// Module: crate::parser::tests::whitespace
// Provides: {"handle_ws_both_sides_for_macro_definitions"}
// Dependencies: {}
# [test] fn handle_ws_both_sides_for_macro_definitions () { let start_ws = WS { left : true , right : true } ; let end_ws = WS { left : true , right : true } ; let ast = vec ! [Node :: MacroDefinition (start_ws , MacroDefinition { name : "something" . to_string () , args : HashMap :: new () , body : vec ! [Node :: Text ("\n  " . to_string ()) , Node :: Text ("hey" . to_string ()) , Node :: Text ("  " . to_string ()) ,] , } , end_ws ,)] ; assert_eq ! (remove_whitespace (ast , None) , vec ! [Node :: MacroDefinition (start_ws , MacroDefinition { name : "something" . to_string () , args : HashMap :: new () , body : vec ! [Node :: Text ("hey" . to_string ())] , } , end_ws ,) ,]) ; }
};
}
