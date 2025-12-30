// Generated macro for cfg_64bit_metrics (macro)
macro_rules! Depcrate_macros_cfgcfg_64bit_metrics {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_64bit_metrics"}
// Dependencies: {}
# [doc = " Some metrics require 64-bit atomics."] macro_rules ! cfg_64bit_metrics { ($ ($ item : item) *) => { $ (# [cfg (target_has_atomic = "64")] # [cfg_attr (docsrs , doc (cfg (target_has_atomic = "64")))] $ item) * } }
};
}
