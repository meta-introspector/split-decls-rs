// Generated macro for imp (module)
macro_rules! Depcrate_sys_local_offset_atimp {
() => {
// Module: crate::sys::local_offset_at
// Provides: {"imp"}
// Dependencies: {}
# [cfg_attr (target_family = "windows" , path = "windows.rs")] # [cfg_attr (target_family = "unix" , path = "unix.rs")] # [cfg_attr (all (target_family = "wasm" , not (any (target_os = "emscripten" , target_os = "wasi")) , feature = "wasm-bindgen") , path = "wasm_js.rs")] mod imp ;
};
}
