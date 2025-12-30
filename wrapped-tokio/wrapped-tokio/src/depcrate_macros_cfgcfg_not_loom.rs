// Generated macro for cfg_not_loom (macro)
macro_rules! Depcrate_macros_cfgcfg_not_loom {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_loom"}
// Dependencies: {}
macro_rules ! cfg_not_loom { ($ ($ item : item) *) => { $ (# [cfg (not (loom))] $ item) * } }
};
}
