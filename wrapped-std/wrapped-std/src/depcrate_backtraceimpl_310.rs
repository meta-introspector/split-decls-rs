// Generated macro for impl_310 (impl)
macro_rules! Depcrate_backtraceimpl_310 {
() => {
// Module: crate::backtrace
// Provides: {"impl_310"}
// Dependencies: {}
impl < 'a > Backtrace { # [doc = " Returns an iterator over the backtrace frames."] # [must_use] # [unstable (feature = "backtrace_frames" , issue = "79676")] pub fn frames (& 'a self) -> & 'a [BacktraceFrame] { if let Inner :: Captured (c) = & self . inner { & c . frames } else { & [] } } }
};
}
