macro_rules! deps {
    () => {
        Node!();
        WS!();
        Expr!();
        ExprVal!();
    };
}

macro_rules! parse_string_concat_can_merge {
    () => {
        deps!();
        # [test] fn parse_string_concat_can_merge () { let ast = parse ("{{ `hello` ~ 'hey' }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: String ("hellohey" . to_string ()))) ,) ; }
    };
}

parse_string_concat_can_merge!()