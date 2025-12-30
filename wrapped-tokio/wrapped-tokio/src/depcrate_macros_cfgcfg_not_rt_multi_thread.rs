// Generated macro for cfg_not_rt_multi_thread (macro)
macro_rules! Depcrate_macros_cfgcfg_not_rt_multi_thread {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_rt_multi_thread"}
// Dependencies: {}
macro_rules ! cfg_not_rt_multi_thread { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "rt-multi-thread"))] $ item) * } }
};
}
