// Generated macro for cfg_io_util (macro)
macro_rules! Depcrate_macroscfg_io_util {
() => {
// Module: crate::macros
// Provides: {"cfg_io_util"}
// Dependencies: {}
macro_rules ! cfg_io_util { ($ ($ item : item) *) => { $ (# [cfg (feature = "io-util")] # [cfg_attr (docsrs , doc (cfg (feature = "io-util")))] $ item) * } }
};
}
