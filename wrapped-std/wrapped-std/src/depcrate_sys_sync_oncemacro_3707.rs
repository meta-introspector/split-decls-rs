// Generated macro for macro_3707 (macro)
macro_rules! Depcrate_sys_sync_oncemacro_3707 {
() => {
// Module: crate::sys::sync::once
// Provides: {"macro_3707"}
// Dependencies: {}
cfg_select ! { any (all (target_os = "windows" , not (target_vendor = "win7")) , target_os = "linux" , target_os = "android" , all (target_arch = "wasm32" , target_feature = "atomics") , target_os = "freebsd" , target_os = "openbsd" , target_os = "dragonfly" , target_os = "fuchsia" , target_os = "hermit" ,) => { mod futex ; pub use futex :: { Once , OnceState } ; } any (windows , target_family = "unix" , all (target_vendor = "fortanix" , target_env = "sgx") , target_os = "solid_asp3" , target_os = "xous" ,) => { mod queue ; pub use queue :: { Once , OnceState } ; } _ => { mod no_threads ; pub use no_threads :: { Once , OnceState } ; } }
};
}
