macro_rules! deps {
    () => {
        Node!();
        ExprVal!();
        Expr!();
        WS!();
    };
}

macro_rules! parse_variable_tag_ident {
    () => {
        deps!();
        # [test] fn parse_variable_tag_ident () { let ast = parse ("{{ id }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Ident ("id" . to_string ()))) ,) ; }
    };
}

parse_variable_tag_ident!();