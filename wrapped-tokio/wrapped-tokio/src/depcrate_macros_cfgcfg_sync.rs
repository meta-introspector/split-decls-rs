// Generated macro for cfg_sync (macro)
macro_rules! Depcrate_macros_cfgcfg_sync {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_sync"}
// Dependencies: {}
macro_rules ! cfg_sync { ($ ($ item : item) *) => { $ (# [cfg (feature = "sync")] # [cfg_attr (docsrs , doc (cfg (feature = "sync")))] $ item) * } }
};
}
