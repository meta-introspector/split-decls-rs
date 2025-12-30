// Generated macro for impl_50 (impl)
macro_rules! Depcrate_astimpl_50 {
() => {
// Module: crate::ast
// Provides: {"impl_50"}
// Dependencies: {}
impl Coproc { # [doc = " Creates a new coprocess."] pub (crate) fn new (name : Option < String > , cmd : impl Into < Cmd >) -> Self { Self { name , cmd : Box :: new (cmd . into ()) , } } }
};
}
