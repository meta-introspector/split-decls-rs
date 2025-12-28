macro_rules! deps {
    () => {
        Node!();
        WS!();
        Expr!();
        ExprVal!();
    };
}

macro_rules! parse_variable_tag_simple_negated_expr {
    () => {
        deps!();
        # [test] fn parse_variable_tag_simple_negated_expr () { let ast = parse ("{{ not id }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new_negated (ExprVal :: Ident ("id" . to_string ())))) ; }
    };
}

parse_variable_tag_simple_negated_expr!();