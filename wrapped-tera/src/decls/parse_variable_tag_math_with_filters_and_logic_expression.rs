macro_rules! deps {
    () => {
        LogicExpr!();
        WS!();
        LogicOperator!();
        Expr!();
        FunctionCall!();
        MathExpr!();
        ExprVal!();
        Node!();
        MathOperator!();
    };
}

macro_rules! parse_variable_tag_math_with_filters_and_logic_expression {
    () => {
        deps!();
        # [test] fn parse_variable_tag_math_with_filters_and_logic_expression () { let ast = parse ("{{ count + 1 * 2.5 | round and admin }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Logic (LogicExpr { lhs : Box :: new (Expr :: with_filters (ExprVal :: Math (MathExpr { lhs : Box :: new (Expr :: new (ExprVal :: Ident ("count" . to_string ()))) , operator : MathOperator :: Add , rhs : Box :: new (Expr :: new (ExprVal :: Math (MathExpr { lhs : Box :: new (Expr :: new (ExprVal :: Int (1))) , operator : MathOperator :: Mul , rhs : Box :: new (Expr :: new (ExprVal :: Float (2.5))) , } ,))) , } ,) , vec ! [FunctionCall { name : "round" . to_string () , args : HashMap :: new () } ,] ,)) , operator : LogicOperator :: And , rhs : Box :: new (Expr :: new (ExprVal :: Ident ("admin" . to_string ()))) , } ,)))) ; }
    };
}

parse_variable_tag_math_with_filters_and_logic_expression!();