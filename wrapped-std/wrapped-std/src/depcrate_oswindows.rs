// Generated macro for windows (module)
macro_rules! Depcrate_oswindows {
() => {
// Module: crate::os
// Provides: {"windows"}
// Dependencies: {}
# [cfg (not (all (doc , any (all (target_arch = "wasm32" , not (target_os = "wasi")) , all (target_vendor = "fortanix" , target_env = "sgx")))))] # [cfg (any (windows , doc))] pub mod windows ;
};
}
