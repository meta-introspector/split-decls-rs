macro_rules! deps {
    () => {
        WS!();
        Expr!();
        StringConcat!();
        ExprVal!();
        Node!();
    };
}

macro_rules! parse_string_concat_multiple {
    () => {
        deps!();
        # [test] fn parse_string_concat_multiple () { let ast = parse ("{{ `hello` ~ ident ~ 'ho' }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: StringConcat (StringConcat { values : vec ! [ExprVal :: String ("hello" . to_string ()) , ExprVal :: Ident ("ident" . to_string ()) , ExprVal :: String ("ho" . to_string ()) ,] }))) ,) ; }
    };
}

parse_string_concat_multiple!();