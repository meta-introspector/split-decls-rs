macro_rules! deps {
    () => {
        Node!();
        Expr!();
        ExprVal!();
        WS!();
    };
}

macro_rules! parse_variable_tag_array_lit {
    () => {
        deps!();
        # [test] fn parse_variable_tag_array_lit () { let ast = parse ("{{ [1, 2, 3] }}") . unwrap () ; let mut join_args = HashMap :: new () ; join_args . insert ("n" . to_string () , Expr :: new (ExprVal :: Int (2))) ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Array (vec ! [Expr :: new (ExprVal :: Int (1)) , Expr :: new (ExprVal :: Int (2)) , Expr :: new (ExprVal :: Int (3))]) ,))) ; }
    };
}

parse_variable_tag_array_lit!()