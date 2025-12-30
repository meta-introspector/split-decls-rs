// Generated macro for cfg_has_atomic_u64 (macro)
macro_rules! Depcrate_macros_cfgcfg_has_atomic_u64 {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_has_atomic_u64"}
// Dependencies: {}
macro_rules ! cfg_has_atomic_u64 { ($ ($ item : item) *) => { $ (# [cfg (target_has_atomic = "64")] $ item) * } }
};
}
