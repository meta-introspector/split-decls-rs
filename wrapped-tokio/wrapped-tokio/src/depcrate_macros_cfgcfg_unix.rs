// Generated macro for cfg_unix (macro)
macro_rules! Depcrate_macros_cfgcfg_unix {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_unix"}
// Dependencies: {}
# [doc = " Enables Unix-specific code."] # [doc = " Use this macro instead of `cfg(unix)` to generate docs properly."] macro_rules ! cfg_unix { ($ ($ item : item) *) => { $ (# [cfg (any (all (doc , docsrs) , unix))] # [cfg_attr (docsrs , doc (cfg (unix)))] $ item) * } }
};
}
