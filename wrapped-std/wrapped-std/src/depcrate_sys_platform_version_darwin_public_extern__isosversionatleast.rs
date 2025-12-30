// Generated macro for __isOSVersionAtLeast (function)
macro_rules! Depcrate_sys_platform_version_darwin_public_extern__isOSVersionAtLeast {
() => {
// Module: crate::sys::platform_version::darwin::public_extern
// Provides: {"__isOSVersionAtLeast"}
// Dependencies: {}
# [doc = " Old entry point for availability. Used when compiling with older Clang versions."] # [rustc_std_internal_symbol] # [linkage = "weak"] pub (super) extern "C" fn __isOSVersionAtLeast (major : i32 , minor : i32 , subminor : i32) -> i32 { let version = pack_i32_os_version (major , minor , subminor) ; (version <= current_version ()) as i32 }
};
}
