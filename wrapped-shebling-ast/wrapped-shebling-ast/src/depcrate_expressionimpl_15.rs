// Generated macro for impl_15 (impl)
macro_rules! Depcrate_expressionimpl_15 {
() => {
// Module: crate::expression
// Provides: {"impl_15"}
// Dependencies: {}
impl < O , E > UnExpr < O , E > { # [doc = " Creates a new unary expression."] pub fn new (expr : impl Into < E > , op : impl Into < O >) -> Self { UnExpr { expr : Box :: new (expr . into ()) , op : op . into () , } } }
};
}
