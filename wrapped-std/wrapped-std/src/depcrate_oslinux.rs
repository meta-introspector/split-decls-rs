// Generated macro for linux (module)
macro_rules! Depcrate_oslinux {
() => {
// Module: crate::os
// Provides: {"linux"}
// Dependencies: {}
# [cfg (not (all (doc , any (all (target_arch = "wasm32" , not (target_os = "wasi")) , all (target_vendor = "fortanix" , target_env = "sgx")))))] # [cfg (any (target_os = "linux" , doc))] pub mod linux ;
};
}
