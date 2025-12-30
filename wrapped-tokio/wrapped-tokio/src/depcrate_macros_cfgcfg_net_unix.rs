// Generated macro for cfg_net_unix (macro)
macro_rules! Depcrate_macros_cfgcfg_net_unix {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_net_unix"}
// Dependencies: {}
macro_rules ! cfg_net_unix { ($ ($ item : item) *) => { $ (# [cfg (all (unix , feature = "net"))] # [cfg_attr (docsrs , doc (cfg (all (unix , feature = "net"))))] $ item) * } }
};
}
