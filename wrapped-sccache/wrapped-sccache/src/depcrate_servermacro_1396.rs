// Generated macro for macro_1396 (macro)
macro_rules! Depcrate_servermacro_1396 {
() => {
// Module: crate::server
// Provides: {"macro_1396"}
// Dependencies: {}
thread_local ! { # [doc = " catch_unwind doesn't provide panic location, so we store that"] # [doc = " information via a panic hook to be used when catch_unwind"] # [doc = " catches a panic."] static PANIC_LOCATION : Cell < Option < (String , u32 , u32) >> = const { Cell :: new (None) } ; }
};
}
