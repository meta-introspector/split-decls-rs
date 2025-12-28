macro_rules! deps {
    () => {
        SyntaxError!();
    };
}

macro_rules! validate_range_expr {
    () => {
        deps!();
        fn validate_range_expr (expr : ast :: RangeExpr , errors : & mut Vec < SyntaxError >) { if expr . op_kind () == Some (ast :: RangeOp :: Inclusive) && expr . end () . is_none () { errors . push (SyntaxError :: new ("An inclusive range must have an end expression" , expr . syntax () . text_range () ,)) ; } }
    };
}

validate_range_expr!()