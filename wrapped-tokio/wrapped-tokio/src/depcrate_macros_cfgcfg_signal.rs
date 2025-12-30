// Generated macro for cfg_signal (macro)
macro_rules! Depcrate_macros_cfgcfg_signal {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_signal"}
// Dependencies: {}
macro_rules ! cfg_signal { ($ ($ item : item) *) => { $ (# [cfg (feature = "signal")] # [cfg_attr (docsrs , doc (cfg (feature = "signal")))] # [cfg (not (loom))] # [cfg (not (target_os = "wasi"))] $ item) * } }
};
}
