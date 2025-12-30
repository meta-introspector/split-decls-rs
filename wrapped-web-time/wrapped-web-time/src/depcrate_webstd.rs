// Generated macro for std (module)
macro_rules! Depcrate_webstd {
() => {
// Module: crate::web
// Provides: {"std"}
// Dependencies: {}
# [cfg (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none") , not (feature = "std") ,))] # [doc (hidden)] mod std { pub mod time { pub struct SystemTime ; } }
};
}
