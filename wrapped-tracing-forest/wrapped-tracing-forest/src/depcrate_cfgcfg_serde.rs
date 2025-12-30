// Generated macro for cfg_serde (macro)
macro_rules! Depcrate_cfgcfg_serde {
() => {
// Module: crate::cfg
// Provides: {"cfg_serde"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! cfg_serde { ($ ($ item : item) *) => { $ (# [cfg (feature = "serde")] # [cfg_attr (docsrs , doc (cfg (feature = "serde")))] $ item) * } }
};
}
