// Generated macro for handle_ws_for_if_nodes (function)
macro_rules! Depcrate_parser_tests_whitespacehandle_ws_for_if_nodes {
() => {
// Module: crate::parser::tests::whitespace
// Provides: {"handle_ws_for_if_nodes"}
// Dependencies: {}
# [test] fn handle_ws_for_if_nodes () { let end_ws = WS { left : false , right : true } ; let ast = vec ! [Node :: Text ("C " . to_string ()) , Node :: If (If { conditions : vec ! [(WS { left : true , right : true } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text (" a " . to_string ())] ,) , (WS { left : true , right : false } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text (" a " . to_string ())] ,) , (WS { left : true , right : true } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text (" a " . to_string ())] ,) ,] , otherwise : None , } , end_ws ,) , Node :: Text ("  hey" . to_string ()) ,] ; assert_eq ! (remove_whitespace (ast , None) , vec ! [Node :: Text ("C" . to_string ()) , Node :: If (If { conditions : vec ! [(WS { left : true , right : true } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text ("a" . to_string ())] ,) , (WS { left : true , right : false } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text (" a" . to_string ())] ,) , (WS { left : true , right : true } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text ("a " . to_string ())] ,) ,] , otherwise : None , } , end_ws ,) , Node :: Text ("hey" . to_string ()) ,]) ; }
};
}
