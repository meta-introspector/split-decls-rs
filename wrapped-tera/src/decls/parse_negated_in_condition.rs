macro_rules! deps {
    () => {
        In!();
        Expr!();
        Node!();
        ExprVal!();
        WS!();
    };
}

macro_rules! parse_negated_in_condition {
    () => {
        deps!();
        # [test] fn parse_negated_in_condition () { let ast = parse ("{{ b not in c }}") . unwrap () ; let mut args = HashMap :: new () ; args . insert ("some" . to_string () , Expr :: new (ExprVal :: Int (1))) ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: In (In { lhs : Box :: new (Expr :: new (ExprVal :: Ident ("b" . to_string ()))) , rhs : Box :: new (Expr :: new (ExprVal :: Ident ("c" . to_string ()))) , negated : true , })))) ; }
    };
}

parse_negated_in_condition!()