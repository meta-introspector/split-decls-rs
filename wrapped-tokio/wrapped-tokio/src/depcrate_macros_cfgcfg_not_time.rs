// Generated macro for cfg_not_time (macro)
macro_rules! Depcrate_macros_cfgcfg_not_time {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_time"}
// Dependencies: {}
macro_rules ! cfg_not_time { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "time"))] $ item) * } }
};
}
