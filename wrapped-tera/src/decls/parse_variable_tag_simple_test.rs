macro_rules! deps {
    () => {
        Node!();
        WS!();
        Expr!();
        ExprVal!();
        Test!();
    };
}

macro_rules! parse_variable_tag_simple_test {
    () => {
        deps!();
        # [test] fn parse_variable_tag_simple_test () { let ast = parse ("{{ id is defined }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Test (Test { ident : "id" . to_string () , negated : false , name : "defined" . to_string () , args : vec ! [] , } ,)))) ; }
    };
}

parse_variable_tag_simple_test!()