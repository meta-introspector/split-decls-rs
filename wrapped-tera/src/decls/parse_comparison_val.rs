macro_rules! deps {
    () => {
        Expr!();
        MathOperator!();
        ExprVal!();
        MathExpr!();
    };
}

macro_rules! parse_comparison_val {
    () => {
        deps!();
        # [doc = " A basic expression with optional filters with prece"] fn parse_comparison_val (pair : Pair < Rule >) -> TeraResult < Expr > { let primary = parse_comparison_val ; let infix = | lhs : TeraResult < Expr > , op : Pair < Rule > , rhs : TeraResult < Expr > | { Ok (Expr :: new (ExprVal :: Math (MathExpr { lhs : Box :: new (lhs ?) , operator : match op . as_rule () { Rule :: op_plus => MathOperator :: Add , Rule :: op_minus => MathOperator :: Sub , Rule :: op_times => MathOperator :: Mul , Rule :: op_slash => MathOperator :: Div , Rule :: op_modulo => MathOperator :: Modulo , _ => unreachable ! () , } , rhs : Box :: new (rhs ?) , }))) } ; let expr = match pair . as_rule () { Rule :: basic_expr_filter => parse_basic_expr_with_filters (pair) ? , Rule :: comparison_val => { MATH_PARSER . map_primary (primary) . map_infix (infix) . parse (pair . into_inner ()) ? } _ => unreachable ! ("Got {:?} in parse_comparison_val" , pair . as_rule ()) , } ; Ok (expr) }
    };
}

parse_comparison_val!();