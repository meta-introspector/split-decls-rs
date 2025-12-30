// Generated macro for cfg_fs (macro)
macro_rules! Depcrate_macroscfg_fs {
() => {
// Module: crate::macros
// Provides: {"cfg_fs"}
// Dependencies: {}
macro_rules ! cfg_fs { ($ ($ item : item) *) => { $ (# [cfg (feature = "fs")] # [cfg_attr (docsrs , doc (cfg (feature = "fs")))] $ item) * } }
};
}
