// Generated macro for cfg_rt (macro)
macro_rules! Depcrate_cfgcfg_rt {
() => {
// Module: crate::cfg
// Provides: {"cfg_rt"}
// Dependencies: {}
macro_rules ! cfg_rt { ($ ($ item : item) *) => { $ (# [cfg (feature = "rt")] # [cfg_attr (docsrs , doc (cfg (feature = "rt")))] $ item) * } }
};
}
