// Generated macro for cfg_not_io_util (macro)
macro_rules! Depcrate_macros_cfgcfg_not_io_util {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_io_util"}
// Dependencies: {}
macro_rules ! cfg_not_io_util { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "io-util"))] $ item) * } }
};
}
