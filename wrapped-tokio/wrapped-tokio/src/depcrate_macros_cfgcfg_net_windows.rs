// Generated macro for cfg_net_windows (macro)
macro_rules! Depcrate_macros_cfgcfg_net_windows {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_net_windows"}
// Dependencies: {}
macro_rules ! cfg_net_windows { ($ ($ item : item) *) => { $ (# [cfg (all (any (all (doc , docsrs) , windows) , feature = "net"))] # [cfg_attr (docsrs , doc (cfg (all (windows , feature = "net"))))] $ item) * } }
};
}
