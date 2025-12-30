// Generated macro for BacktraceStyle (enum)
macro_rules! Depcrate_panicBacktraceStyle {
() => {
// Module: crate::panic
// Provides: {"BacktraceStyle"}
// Dependencies: {}
# [doc = " The configuration for whether and how the default panic hook will capture"] # [doc = " and display the backtrace."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [unstable (feature = "panic_backtrace_config" , issue = "93346")] # [non_exhaustive] pub enum BacktraceStyle { # [doc = " Prints a terser backtrace which ideally only contains relevant"] # [doc = " information."] Short , # [doc = " Prints a backtrace with all possible information."] Full , # [doc = " Disable collecting and displaying backtraces."] Off , }
};
}
