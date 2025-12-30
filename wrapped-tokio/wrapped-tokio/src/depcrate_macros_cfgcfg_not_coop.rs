// Generated macro for cfg_not_coop (macro)
macro_rules! Depcrate_macros_cfgcfg_not_coop {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_coop"}
// Dependencies: {}
macro_rules ! cfg_not_coop { ($ ($ item : item) *) => { $ (# [cfg (not (any (feature = "fs" , feature = "io-std" , feature = "net" , feature = "process" , feature = "rt" , feature = "signal" , feature = "sync" , feature = "time" ,)))] $ item) * } }
};
}
