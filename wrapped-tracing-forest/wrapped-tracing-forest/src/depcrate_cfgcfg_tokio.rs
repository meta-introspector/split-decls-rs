// Generated macro for cfg_tokio (macro)
macro_rules! Depcrate_cfgcfg_tokio {
() => {
// Module: crate::cfg
// Provides: {"cfg_tokio"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! cfg_tokio { ($ ($ item : item) *) => { $ (# [cfg (feature = "tokio")] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] $ item) * } }
};
}
