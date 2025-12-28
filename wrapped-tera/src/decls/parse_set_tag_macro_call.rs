macro_rules! deps {
    () => {
        ExprVal!();
        MacroCall!();
        Expr!();
        Node!();
        Set!();
        WS!();
    };
}

macro_rules! parse_set_tag_macro_call {
    () => {
        deps!();
        # [test] fn parse_set_tag_macro_call () { let ast = parse ("{% set hello = macros::something() %}") . unwrap () ; assert_eq ! (ast [0] , Node :: Set (WS :: default () , Set { key : "hello" . to_string () , value : Expr :: new (ExprVal :: MacroCall (MacroCall { namespace : "macros" . to_string () , name : "something" . to_string () , args : HashMap :: new () , } ,)) , global : false , } ,)) ; }
    };
}

parse_set_tag_macro_call!()