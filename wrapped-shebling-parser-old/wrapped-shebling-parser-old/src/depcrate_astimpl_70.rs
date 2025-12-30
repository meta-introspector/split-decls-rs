// Generated macro for impl_70 (impl)
macro_rules! Depcrate_astimpl_70 {
() => {
// Module: crate::ast
// Provides: {"impl_70"}
// Dependencies: {}
impl InListed { # [doc = " Creates a new group of commands to be executed in an `in lists` loop."] pub (crate) fn new (name : impl Into < String > , list : Vec < Word > , body : impl Into < Term >) -> Self { Self { name : name . into () , list , body : body . into () , } } }
};
}
