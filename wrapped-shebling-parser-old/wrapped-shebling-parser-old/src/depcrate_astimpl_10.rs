// Generated macro for impl_10 (impl)
macro_rules! Depcrate_astimpl_10 {
() => {
// Module: crate::ast
// Provides: {"impl_10"}
// Dependencies: {}
impl < O , E > UnExpr < O , E > { # [doc = " Creates a new unary expression."] pub (crate) fn new (expr : impl Into < E > , op : impl Into < O >) -> Self { Self { expr : Box :: new (expr . into ()) , op : op . into () , } } }
};
}
