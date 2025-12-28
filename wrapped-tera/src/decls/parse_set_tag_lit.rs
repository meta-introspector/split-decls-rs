macro_rules! deps {
    () => {
        Expr!();
        Set!();
        ExprVal!();
        Node!();
        WS!();
    };
}

macro_rules! parse_set_tag_lit {
    () => {
        deps!();
        # [test] fn parse_set_tag_lit () { let ast = parse ("{% set hello = \"hi\" %}") . unwrap () ; assert_eq ! (ast [0] , Node :: Set (WS :: default () , Set { key : "hello" . to_string () , value : Expr :: new (ExprVal :: String ("hi" . to_string ())) , global : false , } ,)) ; }
    };
}

parse_set_tag_lit!();