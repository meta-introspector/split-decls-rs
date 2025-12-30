// Generated macro for FULL_BACKTRACE_DEFAULT (const)
macro_rules! Depcrate_sys_palFULL_BACKTRACE_DEFAULT {
() => {
// Module: crate::sys::pal
// Provides: {"FULL_BACKTRACE_DEFAULT"}
// Dependencies: {}
pub const FULL_BACKTRACE_DEFAULT : bool = cfg_select ! { target_os = "fuchsia" => true , _ => false , } ;
};
}
