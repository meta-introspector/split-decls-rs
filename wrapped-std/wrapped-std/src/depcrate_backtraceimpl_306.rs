// Generated macro for impl_306 (impl)
macro_rules! Depcrate_backtraceimpl_306 {
() => {
// Module: crate::backtrace
// Provides: {"impl_306"}
// Dependencies: {}
# [unstable (feature = "backtrace_frames" , issue = "79676")] impl fmt :: Debug for BacktraceFrame { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut dbg = fmt . debug_list () ; dbg . entries (& self . symbols) ; dbg . finish () } }
};
}
