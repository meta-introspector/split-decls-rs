// Generated macro for impl_3003 (impl)
macro_rules! Depcrate_processimpl_3003 {
() => {
// Module: crate::process
// Provides: {"impl_3003"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] impl From < io :: PipeReader > for Stdio { fn from (pipe : io :: PipeReader) -> Self { Stdio :: from_inner (pipe . into_inner () . into ()) } }
};
}
