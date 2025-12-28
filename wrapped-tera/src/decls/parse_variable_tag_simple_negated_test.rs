macro_rules! deps {
    () => {
        Node!();
        Expr!();
        Test!();
        ExprVal!();
        WS!();
    };
}

macro_rules! parse_variable_tag_simple_negated_test {
    () => {
        deps!();
        # [test] fn parse_variable_tag_simple_negated_test () { let ast = parse ("{{ id is not defined }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Test (Test { ident : "id" . to_string () , negated : true , name : "defined" . to_string () , args : vec ! [] , } ,)))) ; }
    };
}

parse_variable_tag_simple_negated_test!()