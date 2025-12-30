// Generated macro for validate_range_expr (function)
macro_rules! Depcrate_validationvalidate_range_expr {
() => {
// Module: crate::validation
// Provides: {"validate_range_expr"}
// Dependencies: {}
fn validate_range_expr (expr : ast :: RangeExpr , errors : & mut Vec < SyntaxError >) { if expr . op_kind () == Some (ast :: RangeOp :: Inclusive) && expr . end () . is_none () { errors . push (SyntaxError :: new ("An inclusive range must have an end expression" , expr . syntax () . text_range () ,)) ; } }
};
}
