// Generated macro for cfg_is_wasm_not_wasi (macro)
macro_rules! Depcrate_macros_cfgcfg_is_wasm_not_wasi {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_is_wasm_not_wasi"}
// Dependencies: {}
macro_rules ! cfg_is_wasm_not_wasi { ($ ($ item : item) *) => { $ (# [cfg (all (target_family = "wasm" , not (target_os = "wasi")))] $ item) * } }
};
}
