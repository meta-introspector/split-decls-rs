macro_rules! deps {
    () => {
        WS!();
        ExprVal!();
        MathOperator!();
        MathExpr!();
        Expr!();
        Node!();
    };
}

macro_rules! parse_variable_tag_lit_math_expression_with_parentheses {
    () => {
        deps!();
        # [test] fn parse_variable_tag_lit_math_expression_with_parentheses () { let ast = parse ("{{ (count + 1) * 2.5 }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Math (MathExpr { lhs : Box :: new (Expr :: new (ExprVal :: Math (MathExpr { lhs : Box :: new (Expr :: new (ExprVal :: Ident ("count" . to_string ()))) , operator : MathOperator :: Add , rhs : Box :: new (Expr :: new (ExprVal :: Int (1))) , } ,))) , operator : MathOperator :: Mul , rhs : Box :: new (Expr :: new (ExprVal :: Float (2.5))) , } ,)))) ; }
    };
}

parse_variable_tag_lit_math_expression_with_parentheses!()