macro_rules! deps {
    () => {
        LogicOperator!();
        Test!();
        LogicExpr!();
        ExprVal!();
        Node!();
        WS!();
        Expr!();
    };
}

macro_rules! parse_variable_tag_test_as_expression {
    () => {
        deps!();
        # [test] fn parse_variable_tag_test_as_expression () { let ast = parse ("{{ user is defined and user.admin }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Logic (LogicExpr { lhs : Box :: new (Expr :: new (ExprVal :: Test (Test { ident : "user" . to_string () , negated : false , name : "defined" . to_string () , args : vec ! [] , } ,))) , operator : LogicOperator :: And , rhs : Box :: new (Expr :: new (ExprVal :: Ident ("user.admin" . to_string ()))) , } ,)))) ; }
    };
}

parse_variable_tag_test_as_expression!();