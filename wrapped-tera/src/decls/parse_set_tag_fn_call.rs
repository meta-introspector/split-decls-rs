macro_rules! deps {
    () => {
        Set!();
        WS!();
        ExprVal!();
        Node!();
        FunctionCall!();
        Expr!();
    };
}

macro_rules! parse_set_tag_fn_call {
    () => {
        deps!();
        # [test] fn parse_set_tag_fn_call () { let ast = parse ("{% set hello = utcnow() %}") . unwrap () ; assert_eq ! (ast [0] , Node :: Set (WS :: default () , Set { key : "hello" . to_string () , value : Expr :: new (ExprVal :: FunctionCall (FunctionCall { name : "utcnow" . to_string () , args : HashMap :: new () , } ,)) , global : false , } ,)) ; }
    };
}

parse_set_tag_fn_call!()