// Generated macro for cfg_net (macro)
macro_rules! Depcrate_cfgcfg_net {
() => {
// Module: crate::cfg
// Provides: {"cfg_net"}
// Dependencies: {}
macro_rules ! cfg_net { ($ ($ item : item) *) => { $ (# [cfg (all (feature = "net" , feature = "codec"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "net" , feature = "codec"))))] $ item) * } }
};
}
