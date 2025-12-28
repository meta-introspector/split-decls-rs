macro_rules! deps {
    () => {
        FunctionCall!();
        WS!();
        Expr!();
        Node!();
        Set!();
        ExprVal!();
    };
}

macro_rules! parse_set_global_tag {
    () => {
        deps!();
        # [test] fn parse_set_global_tag () { let ast = parse ("{% set_global hello = utcnow() %}") . unwrap () ; assert_eq ! (ast [0] , Node :: Set (WS :: default () , Set { key : "hello" . to_string () , value : Expr :: new (ExprVal :: FunctionCall (FunctionCall { name : "utcnow" . to_string () , args : HashMap :: new () , } ,)) , global : true , } ,)) ; }
    };
}

parse_set_global_tag!();