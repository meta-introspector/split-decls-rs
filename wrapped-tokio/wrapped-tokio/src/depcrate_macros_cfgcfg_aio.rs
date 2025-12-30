// Generated macro for cfg_aio (macro)
macro_rules! Depcrate_macros_cfgcfg_aio {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_aio"}
// Dependencies: {}
macro_rules ! cfg_aio { ($ ($ item : item) *) => { $ (# [cfg (all (any (docsrs , target_os = "freebsd") , feature = "net"))] # [cfg_attr (docsrs , doc (cfg (all (target_os = "freebsd" , feature = "net"))))] $ item) * } }
};
}
