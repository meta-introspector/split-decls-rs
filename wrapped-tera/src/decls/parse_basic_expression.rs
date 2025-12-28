macro_rules! deps {
    () => {
        MathExpr!();
        FunctionCall!();
        Error!();
        Test!();
        ExprVal!();
        Expr!();
        MacroCall!();
        MathOperator!();
    };
}

macro_rules! parse_basic_expression {
    () => {
        deps!();
        fn parse_basic_expression (pair : Pair < Rule >) -> TeraResult < ExprVal > { let primary = parse_basic_expression ; let infix = | lhs : TeraResult < ExprVal > , op : Pair < Rule > , rhs : TeraResult < ExprVal > | { Ok (ExprVal :: Math (MathExpr { lhs : Box :: new (Expr :: new (lhs ?)) , operator : match op . as_rule () { Rule :: op_plus => MathOperator :: Add , Rule :: op_minus => MathOperator :: Sub , Rule :: op_times => MathOperator :: Mul , Rule :: op_slash => MathOperator :: Div , Rule :: op_modulo => MathOperator :: Modulo , _ => unreachable ! () , } , rhs : Box :: new (Expr :: new (rhs ?)) , })) } ; let expr = match pair . as_rule () { Rule :: int => ExprVal :: Int (pair . as_str () . parse () . map_err (| _ | Error :: msg (format ! ("Integer out of bounds: `{}`" , pair . as_str ()))) ? ,) , Rule :: float => ExprVal :: Float (pair . as_str () . parse () . map_err (| _ | Error :: msg (format ! ("Float out of bounds: `{}`" , pair . as_str ()))) ? ,) , Rule :: boolean => match pair . as_str () { "true" => ExprVal :: Bool (true) , "True" => ExprVal :: Bool (true) , "false" => ExprVal :: Bool (false) , "False" => ExprVal :: Bool (false) , _ => unreachable ! () , } , Rule :: test => ExprVal :: Test (parse_test (pair) ?) , Rule :: test_not => { let mut test = parse_test (pair) ? ; test . negated = true ; ExprVal :: Test (test) } Rule :: fn_call => ExprVal :: FunctionCall (parse_fn_call (pair) ?) , Rule :: macro_call => ExprVal :: MacroCall (parse_macro_call (pair) ?) , Rule :: dotted_square_bracket_ident => ExprVal :: Ident (pair . as_str () . to_string ()) , Rule :: basic_expr => { MATH_PARSER . map_primary (primary) . map_infix (infix) . parse (pair . into_inner ()) ? } _ => unreachable ! ("Got {:?} in parse_basic_expression: {}" , pair . as_rule () , pair . as_str ()) , } ; Ok (expr) }
    };
}

parse_basic_expression!()