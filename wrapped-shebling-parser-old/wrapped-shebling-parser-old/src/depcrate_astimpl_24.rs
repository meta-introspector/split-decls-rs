// Generated macro for impl_24 (impl)
macro_rules! Depcrate_astimpl_24 {
() => {
// Module: crate::ast
// Provides: {"impl_24"}
// Dependencies: {}
impl SubscriptedVar { # [doc = " Creates a new subscripted variable with the given identifier"] # [doc = " and subscripts."] pub (crate) fn new (ident : impl Into < String > , subscripts : Vec < String >) -> Self { Self { ident : ident . into () , subscripts , } } }
};
}
