// Generated macro for cfg_process (macro)
macro_rules! Depcrate_macros_cfgcfg_process {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_process"}
// Dependencies: {}
macro_rules ! cfg_process { ($ ($ item : item) *) => { $ (# [cfg (feature = "process")] # [cfg_attr (docsrs , doc (cfg (feature = "process")))] # [cfg (not (loom))] # [cfg (not (target_os = "wasi"))] $ item) * } }
};
}
