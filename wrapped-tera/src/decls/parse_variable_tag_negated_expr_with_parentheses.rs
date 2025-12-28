macro_rules! deps {
    () => {
        Node!();
        ExprVal!();
        MathExpr!();
        MathOperator!();
        LogicOperator!();
        Expr!();
        WS!();
        LogicExpr!();
    };
}

macro_rules! parse_variable_tag_negated_expr_with_parentheses {
    () => {
        deps!();
        # [test] fn parse_variable_tag_negated_expr_with_parentheses () { let ast = parse ("{{ (not id or not true) and not 1 + 1 }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Logic (LogicExpr { lhs : Box :: new (Expr :: new (ExprVal :: Logic (LogicExpr { lhs : Box :: new (Expr :: new_negated (ExprVal :: Ident ("id" . to_string ()))) , operator : LogicOperator :: Or , rhs : Box :: new (Expr :: new_negated (ExprVal :: Bool (true))) , } ,))) , operator : LogicOperator :: And , rhs : Box :: new (Expr :: new_negated (ExprVal :: Math (MathExpr { lhs : Box :: new (Expr :: new (ExprVal :: Int (1))) , operator : MathOperator :: Add , rhs : Box :: new (Expr :: new (ExprVal :: Int (1))) , } ,))) , } ,)))) ; }
    };
}

parse_variable_tag_negated_expr_with_parentheses!()