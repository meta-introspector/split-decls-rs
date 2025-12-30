// Generated macro for impl_3002 (impl)
macro_rules! Depcrate_processimpl_3002 {
() => {
// Module: crate::process
// Provides: {"impl_3002"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] impl From < io :: PipeWriter > for Stdio { fn from (pipe : io :: PipeWriter) -> Self { Stdio :: from_inner (pipe . into_inner () . into ()) } }
};
}
