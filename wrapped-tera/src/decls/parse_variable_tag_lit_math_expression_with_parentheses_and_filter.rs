macro_rules! deps {
    () => {
        ExprVal!();
        FunctionCall!();
        MathExpr!();
        WS!();
        Node!();
        MathOperator!();
        Expr!();
    };
}

macro_rules! parse_variable_tag_lit_math_expression_with_parentheses_and_filter {
    () => {
        deps!();
        # [test] fn parse_variable_tag_lit_math_expression_with_parentheses_and_filter () { let ast = parse ("{{ (count + 1) * 2.5 | round }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: with_filters (ExprVal :: Math (MathExpr { lhs : Box :: new (Expr :: new (ExprVal :: Math (MathExpr { lhs : Box :: new (Expr :: new (ExprVal :: Ident ("count" . to_string ()))) , operator : MathOperator :: Add , rhs : Box :: new (Expr :: new (ExprVal :: Int (1))) , } ,))) , operator : MathOperator :: Mul , rhs : Box :: new (Expr :: new (ExprVal :: Float (2.5))) , } ,) , vec ! [FunctionCall { name : "round" . to_string () , args : HashMap :: new () } ,] ,))) ; }
    };
}

parse_variable_tag_lit_math_expression_with_parentheses_and_filter!()