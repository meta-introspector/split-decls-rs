macro_rules! deps {
    () => {
        Node!();
        ExprVal!();
        WS!();
        MacroCall!();
        Expr!();
    };
}

macro_rules! parse_variable_tag_macro_call {
    () => {
        deps!();
        # [test] fn parse_variable_tag_macro_call () { let ast = parse ("{{ macros::get_time(some=1) }}") . unwrap () ; let mut args = HashMap :: new () ; args . insert ("some" . to_string () , Expr :: new (ExprVal :: Int (1))) ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: MacroCall (MacroCall { namespace : "macros" . to_string () , name : "get_time" . to_string () , args , } ,)) ,)) ; }
    };
}

parse_variable_tag_macro_call!();