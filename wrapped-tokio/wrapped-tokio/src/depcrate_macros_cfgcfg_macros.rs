// Generated macro for cfg_macros (macro)
macro_rules! Depcrate_macros_cfgcfg_macros {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_macros"}
// Dependencies: {}
macro_rules ! cfg_macros { ($ ($ item : item) *) => { $ (# [cfg (feature = "macros")] # [cfg_attr (docsrs , doc (cfg (feature = "macros")))] $ item) * } }
};
}
