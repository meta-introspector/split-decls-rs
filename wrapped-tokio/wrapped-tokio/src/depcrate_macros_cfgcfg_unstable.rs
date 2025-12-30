// Generated macro for cfg_unstable (macro)
macro_rules! Depcrate_macros_cfgcfg_unstable {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_unstable"}
// Dependencies: {}
macro_rules ! cfg_unstable { ($ ($ item : item) *) => { $ (# [cfg (tokio_unstable)] # [cfg_attr (docsrs , doc (cfg (tokio_unstable)))] $ item) * } ; }
};
}
