macro_rules! deps {
    () => {
        WS!();
        Node!();
        If!();
        ExprVal!();
        Expr!();
    };
}

macro_rules! handle_ws_for_if_nodes {
    () => {
        deps!();
        # [test] fn handle_ws_for_if_nodes () { let end_ws = WS { left : false , right : true } ; let ast = vec ! [Node :: Text ("C " . to_string ()) , Node :: If (If { conditions : vec ! [(WS { left : true , right : true } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text (" a " . to_string ())] ,) , (WS { left : true , right : false } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text (" a " . to_string ())] ,) , (WS { left : true , right : true } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text (" a " . to_string ())] ,) ,] , otherwise : None , } , end_ws ,) , Node :: Text ("  hey" . to_string ()) ,] ; assert_eq ! (remove_whitespace (ast , None) , vec ! [Node :: Text ("C" . to_string ()) , Node :: If (If { conditions : vec ! [(WS { left : true , right : true } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text ("a" . to_string ())] ,) , (WS { left : true , right : false } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text (" a" . to_string ())] ,) , (WS { left : true , right : true } , Expr :: new (ExprVal :: Int (1)) , vec ! [Node :: Text ("a " . to_string ())] ,) ,] , otherwise : None , } , end_ws ,) , Node :: Text ("hey" . to_string ()) ,]) ; }
    };
}

handle_ws_for_if_nodes!();