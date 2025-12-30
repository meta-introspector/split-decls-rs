// Generated macro for cfg_rt_multi_thread (macro)
macro_rules! Depcrate_macros_cfgcfg_rt_multi_thread {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_rt_multi_thread"}
// Dependencies: {}
macro_rules ! cfg_rt_multi_thread { ($ ($ item : item) *) => { $ (# [cfg (feature = "rt-multi-thread")] # [cfg_attr (docsrs , doc (cfg (feature = "rt-multi-thread")))] $ item) * } }
};
}
