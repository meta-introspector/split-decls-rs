macro_rules! deps {
    () => {
        LogicOperator!();
        ExprVal!();
        LogicExpr!();
        Expr!();
    };
}

macro_rules! parse_logic_expr {
    () => {
        deps!();
        fn parse_logic_expr (pair : Pair < Rule >) -> TeraResult < Expr > { let primary = parse_logic_expr ; let infix = | lhs : TeraResult < Expr > , op : Pair < Rule > , rhs : TeraResult < Expr > | match op . as_rule () { Rule :: op_or => Ok (Expr :: new (ExprVal :: Logic (LogicExpr { lhs : Box :: new (lhs ?) , operator : LogicOperator :: Or , rhs : Box :: new (rhs ?) , }))) , Rule :: op_and => Ok (Expr :: new (ExprVal :: Logic (LogicExpr { lhs : Box :: new (lhs ?) , operator : LogicOperator :: And , rhs : Box :: new (rhs ?) , }))) , _ => unreachable ! ("{:?} not supposed to get there (infix of logic_expression)!" , op . as_rule ()) , } ; let expr = match pair . as_rule () { Rule :: logic_val => parse_logic_val (pair) ? , Rule :: logic_expr => { LOGIC_EXPR_PARSER . map_primary (primary) . map_infix (infix) . parse (pair . into_inner ()) ? } _ => unreachable ! ("Got {:?} in parse_logic_expr" , pair . as_rule ()) , } ; Ok (expr) }
    };
}

parse_logic_expr!();