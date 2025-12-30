// Generated macro for handle_ws_for_if_nodes_with_else (function)
macro_rules! Depcrate_parser_tests_whitespacehandle_ws_for_if_nodes_with_else {
() => {
// Module: crate::parser::tests::whitespace
// Provides: {"handle_ws_for_if_nodes_with_else"}
// Dependencies: {}
# [test] fn handle_ws_for_if_nodes_with_else () { let end_ws = WS { left : true , right : true } ; let ast = vec ! [Node :: Text ("C " . to_string ()) , Node :: If (If { conditions : vec ! [(WS { left : true , right : true } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text (" a " . to_string ())] ,) , (WS { left : true , right : false } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text (" a " . to_string ())] ,) , (WS { left : true , right : true } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text (" a " . to_string ())] ,) ,] , otherwise : Some ((WS { left : true , right : true } , vec ! [Node :: Text (" a " . to_string ())] ,)) , } , end_ws ,) , Node :: Text ("  hey" . to_string ()) ,] ; assert_eq ! (remove_whitespace (ast , None) , vec ! [Node :: Text ("C" . to_string ()) , Node :: If (If { conditions : vec ! [(WS { left : true , right : true } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text ("a" . to_string ())] ,) , (WS { left : true , right : false } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text (" a" . to_string ())] ,) , (WS { left : true , right : true } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text ("a" . to_string ())] ,) ,] , otherwise : Some ((WS { left : true , right : true } , vec ! [Node :: Text ("a" . to_string ())] ,)) , } , end_ws ,) , Node :: Text ("hey" . to_string ()) ,]) ; }
};
}
