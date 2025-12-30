// Generated macro for Backtrace (struct)
macro_rules! Depcrate_backtraceBacktrace {
() => {
// Module: crate::backtrace
// Provides: {"Backtrace"}
// Dependencies: {}
# [doc = " A captured OS thread stack backtrace."] # [doc = ""] # [doc = " This type represents a stack backtrace for an OS thread captured at a"] # [doc = " previous point in time. In some instances the `Backtrace` type may"] # [doc = " internally be empty due to configuration. For more information see"] # [doc = " `Backtrace::capture`."] # [stable (feature = "backtrace" , since = "1.65.0")] # [must_use] pub struct Backtrace { inner : Inner , }
};
}
