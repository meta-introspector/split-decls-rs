// Generated macro for impl_68 (impl)
macro_rules! Depcrate_astimpl_68 {
() => {
// Module: crate::ast
// Provides: {"impl_68"}
// Dependencies: {}
impl ArithForLoop { # [doc = " Creates a new arithmetic for-loop."] pub (crate) fn new (header : (ArithSeq , ArithSeq , ArithSeq) , body : impl Into < Term >) -> Self { Self { header , body : body . into () , } } }
};
}
