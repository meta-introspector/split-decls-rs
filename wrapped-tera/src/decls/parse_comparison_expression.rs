macro_rules! deps {
    () => {
        ExprVal!();
        Expr!();
        LogicOperator!();
        LogicExpr!();
    };
}

macro_rules! parse_comparison_expression {
    () => {
        deps!();
        fn parse_comparison_expression (pair : Pair < Rule >) -> TeraResult < Expr > { let primary = parse_comparison_expression ; let infix = | lhs : TeraResult < Expr > , op : Pair < Rule > , rhs : TeraResult < Expr > | { Ok (Expr :: new (ExprVal :: Logic (LogicExpr { lhs : Box :: new (lhs ?) , operator : match op . as_rule () { Rule :: op_lt => LogicOperator :: Lt , Rule :: op_lte => LogicOperator :: Lte , Rule :: op_gt => LogicOperator :: Gt , Rule :: op_gte => LogicOperator :: Gte , Rule :: op_ineq => LogicOperator :: NotEq , Rule :: op_eq => LogicOperator :: Eq , _ => unreachable ! () , } , rhs : Box :: new (rhs ?) , }))) } ; let expr = match pair . as_rule () { Rule :: comparison_val => parse_comparison_val (pair) ? , Rule :: string_expr_filter => parse_string_expr_with_filters (pair) ? , Rule :: comparison_expr => { COMPARISON_EXPR_PARSER . map_primary (primary) . map_infix (infix) . parse (pair . into_inner ()) ? } _ => unreachable ! ("Got {:?} in parse_comparison_expression" , pair . as_rule ()) , } ; Ok (expr) }
    };
}

parse_comparison_expression!()