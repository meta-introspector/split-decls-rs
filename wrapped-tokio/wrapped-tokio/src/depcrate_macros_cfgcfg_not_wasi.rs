// Generated macro for cfg_not_wasi (macro)
macro_rules! Depcrate_macros_cfgcfg_not_wasi {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_wasi"}
// Dependencies: {}
macro_rules ! cfg_not_wasi { ($ ($ item : item) *) => { $ (# [cfg (not (target_os = "wasi"))] $ item) * } }
};
}
