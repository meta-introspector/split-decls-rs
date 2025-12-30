// Generated macro for wasi (module)
macro_rules! Depcrate_oswasi {
() => {
// Module: crate::os
// Provides: {"wasi"}
// Dependencies: {}
# [cfg (not (all (doc , any (all (target_arch = "wasm32" , not (target_os = "wasi")) , all (target_vendor = "fortanix" , target_env = "sgx")))))] # [cfg (any (target_os = "wasi" , doc))] pub mod wasi ;
};
}
