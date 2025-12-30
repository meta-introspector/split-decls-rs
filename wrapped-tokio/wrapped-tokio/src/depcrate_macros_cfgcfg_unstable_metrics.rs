// Generated macro for cfg_unstable_metrics (macro)
macro_rules! Depcrate_macros_cfgcfg_unstable_metrics {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_unstable_metrics"}
// Dependencies: {}
macro_rules ! cfg_unstable_metrics { ($ ($ item : item) *) => { $ (# [cfg (tokio_unstable)] # [cfg_attr (docsrs , doc (cfg (tokio_unstable)))] $ item) * } }
};
}
