// Generated macro for unix (module)
macro_rules! Depcrate_osunix {
() => {
// Module: crate::os
// Provides: {"unix"}
// Dependencies: {}
# [cfg (not (all (doc , any (all (target_arch = "wasm32" , not (target_os = "wasi")) , all (target_vendor = "fortanix" , target_env = "sgx")))))] # [cfg (all (not (target_os = "hermit") , any (unix , doc)))] pub mod unix ;
};
}
