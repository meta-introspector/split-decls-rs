// Generated macro for BacktraceFrame (struct)
macro_rules! Depcrate_backtraceBacktraceFrame {
() => {
// Module: crate::backtrace
// Provides: {"BacktraceFrame"}
// Dependencies: {}
# [doc = " A single frame of a backtrace."] # [unstable (feature = "backtrace_frames" , issue = "79676")] pub struct BacktraceFrame { frame : RawFrame , symbols : Vec < BacktraceSymbol > , }
};
}
