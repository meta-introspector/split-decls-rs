// Generated macro for impl_447 (impl)
macro_rules! Depcrateimpl_447 {
() => {
// Module: crate
// Provides: {"impl_447"}
// Dependencies: {}
impl ErrorGuaranteed { # [doc = " Don't use this outside of `DiagCtxtInner::emit_diagnostic`!"] # [deprecated = "should only be used in `DiagCtxtInner::emit_diagnostic`"] pub fn unchecked_error_guaranteed () -> Self { ErrorGuaranteed (()) } pub fn raise_fatal (self) -> ! { FatalError . raise () } }
};
}
