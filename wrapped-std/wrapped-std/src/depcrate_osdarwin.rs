// Generated macro for darwin (module)
macro_rules! Depcrate_osdarwin {
() => {
// Module: crate::os
// Provides: {"darwin"}
// Dependencies: {}
# [cfg (not (all (doc , any (all (target_arch = "wasm32" , not (target_os = "wasi")) , all (target_vendor = "fortanix" , target_env = "sgx")))))] # [cfg (any (target_vendor = "apple" , doc))] pub mod darwin ;
};
}
