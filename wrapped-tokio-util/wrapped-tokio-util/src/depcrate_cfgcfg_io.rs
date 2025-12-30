// Generated macro for cfg_io (macro)
macro_rules! Depcrate_cfgcfg_io {
() => {
// Module: crate::cfg
// Provides: {"cfg_io"}
// Dependencies: {}
macro_rules ! cfg_io { ($ ($ item : item) *) => { $ (# [cfg (feature = "io")] # [cfg_attr (docsrs , doc (cfg (feature = "io")))] $ item) * } }
};
}
