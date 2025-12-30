// Generated macro for cfg_not_rt (macro)
macro_rules! Depcrate_macros_cfgcfg_not_rt {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_rt"}
// Dependencies: {}
macro_rules ! cfg_not_rt { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "rt"))] $ item) * } }
};
}
