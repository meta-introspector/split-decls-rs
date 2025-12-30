// Generated macro for handle_ws_both_sides_for_forloop_tag_and_remove_empty_node (function)
macro_rules! Depcrate_parser_tests_whitespacehandle_ws_both_sides_for_forloop_tag_and_remove_empty_node {
() => {
// Module: crate::parser::tests::whitespace
// Provides: {"handle_ws_both_sides_for_forloop_tag_and_remove_empty_node"}
// Dependencies: {}
# [test] fn handle_ws_both_sides_for_forloop_tag_and_remove_empty_node () { let start_ws = WS { left : true , right : true } ; let end_ws = WS { left : true , right : true } ; let ast = vec ! [Node :: Forloop (start_ws , Forloop { key : None , value : "item" . to_string () , container : Expr :: new (ExprVal :: Int (1)) , body : vec ! [Node :: Text ("   " . to_string ()) , Node :: Text ("hey   " . to_string ())] , empty_body : None , } , end_ws ,) , Node :: Text ("  hey" . to_string ()) ,] ; assert_eq ! (remove_whitespace (ast , None) , vec ! [Node :: Forloop (start_ws , Forloop { key : None , value : "item" . to_string () , container : Expr :: new (ExprVal :: Int (1)) , body : vec ! [Node :: Text ("hey" . to_string ())] , empty_body : None , } , end_ws ,) , Node :: Text ("hey" . to_string ()) ,]) ; }
};
}
