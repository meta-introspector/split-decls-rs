// Generated macro for BacktraceStatus (enum)
macro_rules! Depcrate_backtraceBacktraceStatus {
() => {
// Module: crate::backtrace
// Provides: {"BacktraceStatus"}
// Dependencies: {}
# [doc = " The current status of a backtrace, indicating whether it was captured or"] # [doc = " whether it is empty for some other reason."] # [stable (feature = "backtrace" , since = "1.65.0")] # [non_exhaustive] # [derive (Debug , PartialEq , Eq)] pub enum BacktraceStatus { # [doc = " Capturing a backtrace is not supported, likely because it's not"] # [doc = " implemented for the current platform."] # [stable (feature = "backtrace" , since = "1.65.0")] Unsupported , # [doc = " Capturing a backtrace has been disabled through either the"] # [doc = " `RUST_LIB_BACKTRACE` or `RUST_BACKTRACE` environment variables."] # [stable (feature = "backtrace" , since = "1.65.0")] Disabled , # [doc = " A backtrace has been captured and the `Backtrace` should print"] # [doc = " reasonable information when rendered."] # [stable (feature = "backtrace" , since = "1.65.0")] Captured , }
};
}
