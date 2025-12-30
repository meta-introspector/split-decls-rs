// Generated macro for cfg_net_or_process (macro)
macro_rules! Depcrate_macros_cfgcfg_net_or_process {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_net_or_process"}
// Dependencies: {}
macro_rules ! cfg_net_or_process { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "net" , feature = "process"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "net" , feature = "process"))))] $ item) * } }
};
}
