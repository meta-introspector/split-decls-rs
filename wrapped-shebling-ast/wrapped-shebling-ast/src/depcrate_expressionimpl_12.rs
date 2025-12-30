// Generated macro for impl_12 (impl)
macro_rules! Depcrate_expressionimpl_12 {
() => {
// Module: crate::expression
// Provides: {"impl_12"}
// Dependencies: {}
impl < O , L , R > BinExpr < O , L , R > { # [doc = " Creates a new binary expression."] pub fn new (left : impl Into < L > , right : impl Into < R > , op : impl Into < O >) -> Self { BinExpr { left : Box :: new (left . into ()) , right : Box :: new (right . into ()) , op : op . into () , } } }
};
}
