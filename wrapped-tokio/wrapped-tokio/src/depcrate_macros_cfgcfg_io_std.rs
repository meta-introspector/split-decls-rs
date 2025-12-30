// Generated macro for cfg_io_std (macro)
macro_rules! Depcrate_macros_cfgcfg_io_std {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_io_std"}
// Dependencies: {}
macro_rules ! cfg_io_std { ($ ($ item : item) *) => { $ (# [cfg (feature = "io-std")] # [cfg_attr (docsrs , doc (cfg (feature = "io-std")))] $ item) * } }
};
}
