macro_rules! deps {
    () => {
        WS!();
        MacroCall!();
        ExprVal!();
        Expr!();
        FunctionCall!();
        Node!();
    };
}

macro_rules! parse_variable_tag_macro_call_with_array_with_filters {
    () => {
        deps!();
        # [test] fn parse_variable_tag_macro_call_with_array_with_filters () { let ast = parse ("{{ macros::get_time(some=[1, 2] | reverse) }}") . unwrap () ; let mut args = HashMap :: new () ; args . insert ("some" . to_string () , Expr :: with_filters (ExprVal :: Array (vec ! [Expr :: new (ExprVal :: Int (1)) , Expr :: new (ExprVal :: Int (2))]) , vec ! [FunctionCall { name : "reverse" . to_string () , args : HashMap :: new () }] ,) ,) ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: MacroCall (MacroCall { namespace : "macros" . to_string () , name : "get_time" . to_string () , args , } ,)))) ; }
    };
}

parse_variable_tag_macro_call_with_array_with_filters!()