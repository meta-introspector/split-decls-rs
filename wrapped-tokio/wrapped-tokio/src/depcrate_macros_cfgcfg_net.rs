// Generated macro for cfg_net (macro)
macro_rules! Depcrate_macros_cfgcfg_net {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_net"}
// Dependencies: {}
macro_rules ! cfg_net { ($ ($ item : item) *) => { $ (# [cfg (feature = "net")] # [cfg_attr (docsrs , doc (cfg (feature = "net")))] $ item) * } }
};
}
