// Generated macro for impl_48 (impl)
macro_rules! Depcrate_astimpl_48 {
() => {
// Module: crate::ast
// Provides: {"impl_48"}
// Dependencies: {}
impl CompoundCmd { # [doc = " Creates a new compound command."] pub (crate) fn new (cmd : impl Into < Construct > , redirs : Vec < Redir >) -> Self { Self { cmd : cmd . into () , redirs , } } }
};
}
