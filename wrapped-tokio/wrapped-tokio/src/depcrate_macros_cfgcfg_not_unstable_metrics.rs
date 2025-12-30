// Generated macro for cfg_not_unstable_metrics (macro)
macro_rules! Depcrate_macros_cfgcfg_not_unstable_metrics {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_unstable_metrics"}
// Dependencies: {}
macro_rules ! cfg_not_unstable_metrics { ($ ($ item : item) *) => { $ (# [cfg (not (tokio_unstable))] $ item) * } }
};
}
