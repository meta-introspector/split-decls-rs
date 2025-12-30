// Generated macro for impl_314 (impl)
macro_rules! Depcrate_backtraceimpl_314 {
() => {
// Module: crate::backtrace
// Provides: {"impl_314"}
// Dependencies: {}
impl RawFrame { fn ip (& self) -> * mut c_void { match self { RawFrame :: Actual (frame) => frame . ip () , # [cfg (test)] RawFrame :: Fake => crate :: ptr :: without_provenance_mut (1) , } } }
};
}
