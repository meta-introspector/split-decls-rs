// Generated macro for impl_7 (impl)
macro_rules! Depcrate_astimpl_7 {
() => {
// Module: crate::ast
// Provides: {"impl_7"}
// Dependencies: {}
impl < O , L , R > BinExpr < O , L , R > { # [doc = " Creates a new binary expression."] pub (crate) fn new (left : impl Into < L > , right : impl Into < R > , op : impl Into < O >) -> Self { Self { left : Box :: new (left . into ()) , right : Box :: new (right . into ()) , op : op . into () , } } }
};
}
