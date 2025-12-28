macro_rules! deps {
    () => {
        Node!();
        WS!();
        Expr!();
        ExprVal!();
    };
}

macro_rules! parse_variable_with_whitespace_trimming {
    () => {
        deps!();
        # [test] fn parse_variable_with_whitespace_trimming () { let ast = parse ("{{- id }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS { left : true , right : false } , Expr :: new (ExprVal :: Ident ("id" . to_string ()))) ,) ; }
    };
}

parse_variable_with_whitespace_trimming!();