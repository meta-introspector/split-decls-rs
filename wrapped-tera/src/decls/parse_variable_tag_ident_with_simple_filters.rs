macro_rules! deps {
    () => {
        WS!();
        Expr!();
        Node!();
        ExprVal!();
        FunctionCall!();
    };
}

macro_rules! parse_variable_tag_ident_with_simple_filters {
    () => {
        deps!();
        # [test] fn parse_variable_tag_ident_with_simple_filters () { let ast = parse ("{{ arr | first | join(n=2) }}") . unwrap () ; let mut join_args = HashMap :: new () ; join_args . insert ("n" . to_string () , Expr :: new (ExprVal :: Int (2))) ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: with_filters (ExprVal :: Ident ("arr" . to_string ()) , vec ! [FunctionCall { name : "first" . to_string () , args : HashMap :: new () } , FunctionCall { name : "join" . to_string () , args : join_args } ,] ,))) ; }
    };
}

parse_variable_tag_ident_with_simple_filters!()