// Generated macro for cfg_no_64bit_metrics (macro)
macro_rules! Depcrate_macros_cfgcfg_no_64bit_metrics {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_no_64bit_metrics"}
// Dependencies: {}
macro_rules ! cfg_no_64bit_metrics { ($ ($ item : item) *) => { $ (# [cfg (not (target_has_atomic = "64"))] $ item) * } }
};
}
