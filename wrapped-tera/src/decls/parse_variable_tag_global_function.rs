macro_rules! deps {
    () => {
        Expr!();
        Node!();
        WS!();
        ExprVal!();
        FunctionCall!();
    };
}

macro_rules! parse_variable_tag_global_function {
    () => {
        deps!();
        # [test] fn parse_variable_tag_global_function () { let ast = parse ("{{ get_time(some=1) }}") . unwrap () ; let mut args = HashMap :: new () ; args . insert ("some" . to_string () , Expr :: new (ExprVal :: Int (1))) ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: FunctionCall (FunctionCall { name : "get_time" . to_string () , args } ,)))) ; }
    };
}

parse_variable_tag_global_function!()