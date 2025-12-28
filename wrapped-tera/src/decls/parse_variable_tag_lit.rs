macro_rules! deps {
    () => {
        Node!();
        WS!();
        ExprVal!();
        Expr!();
    };
}

macro_rules! parse_variable_tag_lit {
    () => {
        deps!();
        # [test] fn parse_variable_tag_lit () { let ast = parse ("{{ 2 }}{{ 3.18 }}{{ \"hey\" }}{{ true }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Int (2)))) ; assert_eq ! (ast [1] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Float (3.18)))) ; assert_eq ! (ast [2] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: String ("hey" . to_string ()))) ,) ; assert_eq ! (ast [3] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Bool (true)))) ; }
    };
}

parse_variable_tag_lit!();