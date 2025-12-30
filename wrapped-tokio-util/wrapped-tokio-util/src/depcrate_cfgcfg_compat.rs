// Generated macro for cfg_compat (macro)
macro_rules! Depcrate_cfgcfg_compat {
() => {
// Module: crate::cfg
// Provides: {"cfg_compat"}
// Dependencies: {}
macro_rules ! cfg_compat { ($ ($ item : item) *) => { $ (# [cfg (feature = "compat")] # [cfg_attr (docsrs , doc (cfg (feature = "compat")))] $ item) * } }
};
}
