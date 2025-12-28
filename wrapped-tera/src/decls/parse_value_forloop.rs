macro_rules! deps {
    () => {
        FunctionCall!();
        Node!();
        WS!();
        ExprVal!();
        Forloop!();
        Expr!();
    };
}

macro_rules! parse_value_forloop {
    () => {
        deps!();
        # [test] fn parse_value_forloop () { let ast = parse ("{% for item in items | reverse %}A{%- endfor %}") . unwrap () ; let start_ws = WS :: default () ; let end_ws = WS { left : true , .. Default :: default () } ; assert_eq ! (ast [0] , Node :: Forloop (start_ws , Forloop { key : None , value : "item" . to_string () , container : Expr :: with_filters (ExprVal :: Ident ("items" . to_string ()) , vec ! [FunctionCall { name : "reverse" . to_string () , args : HashMap :: new () } ,] ,) , body : vec ! [Node :: Text ("A" . to_string ())] , empty_body : None , } , end_ws ,)) ; }
    };
}

parse_value_forloop!();