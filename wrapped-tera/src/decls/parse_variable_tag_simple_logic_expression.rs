macro_rules! deps {
    () => {
        LogicExpr!();
        Node!();
        Expr!();
        ExprVal!();
        LogicOperator!();
        WS!();
    };
}

macro_rules! parse_variable_tag_simple_logic_expression {
    () => {
        deps!();
        # [test] fn parse_variable_tag_simple_logic_expression () { let ast = parse ("{{ 1 > 2 }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Logic (LogicExpr { lhs : Box :: new (Expr :: new (ExprVal :: Int (1))) , operator : LogicOperator :: Gt , rhs : Box :: new (Expr :: new (ExprVal :: Int (2))) , } ,)))) ; }
    };
}

parse_variable_tag_simple_logic_expression!()