// Generated macro for impl_76 (impl)
macro_rules! Depcrate_astimpl_76 {
() => {
// Module: crate::ast
// Provides: {"impl_76"}
// Dependencies: {}
impl CondBlock { # [doc = " Creates a new conditional block."] pub (crate) fn new (cond : impl Into < Term > , block : impl Into < Term >) -> Self { Self { cond : cond . into () , block : block . into () , } } }
};
}
