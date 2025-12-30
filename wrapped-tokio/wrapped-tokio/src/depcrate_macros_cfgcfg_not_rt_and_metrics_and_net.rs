// Generated macro for cfg_not_rt_and_metrics_and_net (macro)
macro_rules! Depcrate_macros_cfgcfg_not_rt_and_metrics_and_net {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_rt_and_metrics_and_net"}
// Dependencies: {}
macro_rules ! cfg_not_rt_and_metrics_and_net { ($ ($ item : item) *) => { $ (# [cfg (not (all (feature = "net" , feature = "rt" , tokio_unstable)))] $ item) * } }
};
}
