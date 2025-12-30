// Generated macro for cfg_not_sync (macro)
macro_rules! Depcrate_macros_cfgcfg_not_sync {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_sync"}
// Dependencies: {}
macro_rules ! cfg_not_sync { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "sync"))] $ item) * } }
};
}
