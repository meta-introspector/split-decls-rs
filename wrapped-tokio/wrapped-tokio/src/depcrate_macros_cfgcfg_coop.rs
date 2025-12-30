// Generated macro for cfg_coop (macro)
macro_rules! Depcrate_macros_cfgcfg_coop {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_coop"}
// Dependencies: {}
macro_rules ! cfg_coop { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "fs" , feature = "io-std" , feature = "net" , feature = "process" , feature = "rt" , feature = "signal" , feature = "sync" , feature = "time" ,))] $ item) * } }
};
}
