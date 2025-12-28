macro_rules! deps {
    () => {
        Expr!();
        ExprVal!();
        WS!();
        Node!();
        Set!();
    };
}

macro_rules! parse_set_array {
    () => {
        deps!();
        # [test] fn parse_set_array () { let ast = parse ("{% set hello = [1, true, 'hello'] %}") . unwrap () ; assert_eq ! (ast [0] , Node :: Set (WS :: default () , Set { key : "hello" . to_string () , value : Expr :: new (ExprVal :: Array (vec ! [Expr :: new (ExprVal :: Int (1)) , Expr :: new (ExprVal :: Bool (true)) , Expr :: new (ExprVal :: String ("hello" . to_string ())) ,])) , global : false , } ,)) ; }
    };
}

parse_set_array!()