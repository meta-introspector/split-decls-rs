// Generated macro for set_backtrace_style (function)
macro_rules! Depcrate_panicset_backtrace_style {
() => {
// Module: crate::panic
// Provides: {"set_backtrace_style"}
// Dependencies: {}
# [doc = " Configures whether the default panic hook will capture and display a"] # [doc = " backtrace."] # [doc = ""] # [doc = " The default value for this setting may be set by the `RUST_BACKTRACE`"] # [doc = " environment variable; see the details in [`get_backtrace_style`]."] # [unstable (feature = "panic_backtrace_config" , issue = "93346")] pub fn set_backtrace_style (style : BacktraceStyle) { if cfg ! (feature = "backtrace") { SHOULD_CAPTURE . store (style . as_u8 () , Ordering :: Relaxed) ; } }
};
}
