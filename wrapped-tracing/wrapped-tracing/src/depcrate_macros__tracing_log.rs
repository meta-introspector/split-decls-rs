// Generated macro for __tracing_log (macro)
macro_rules! Depcrate_macros__tracing_log {
() => {
// Module: crate::macros
// Provides: {"__tracing_log"}
// Dependencies: {}
# [cfg (feature = "log")] # [doc (hidden)] # [macro_export] macro_rules ! __tracing_log { ($ level : expr , $ callsite : expr , $ value_set : expr) => { $ crate :: if_log_enabled ! { $ level , { use $ crate :: log ; let level = $ crate :: level_to_log ! ($ level) ; if level <= log :: max_level () { let meta = $ callsite . metadata () ; let log_meta = log :: Metadata :: builder () . level (level) . target (meta . target ()) . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { $ crate :: __macro_support :: __tracing_log (meta , logger , log_meta , $ value_set) } } } } } ; }
};
}
