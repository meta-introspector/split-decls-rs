// Generated macro for cfg_io_util (macro)
macro_rules! Depcrate_macros_cfgcfg_io_util {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_io_util"}
// Dependencies: {}
macro_rules ! cfg_io_util { ($ ($ item : item) *) => { $ (# [cfg (feature = "io-util")] # [cfg_attr (docsrs , doc (cfg (feature = "io-util")))] $ item) * } }
};
}
