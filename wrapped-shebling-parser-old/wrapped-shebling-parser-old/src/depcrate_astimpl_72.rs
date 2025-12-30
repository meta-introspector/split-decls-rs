// Generated macro for impl_72 (impl)
macro_rules! Depcrate_astimpl_72 {
() => {
// Module: crate::ast
// Provides: {"impl_72"}
// Dependencies: {}
impl Function { # [doc = " Creates a new function."] pub (crate) fn new (name : impl Into < String > , body : impl Into < Term >) -> Self { Self { name : name . into () , body : body . into () , } } }
};
}
