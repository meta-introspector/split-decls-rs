// Generated macro for cfg_time (macro)
macro_rules! Depcrate_cfgcfg_time {
() => {
// Module: crate::cfg
// Provides: {"cfg_time"}
// Dependencies: {}
macro_rules ! cfg_time { ($ ($ item : item) *) => { $ (# [cfg (feature = "time")] # [cfg_attr (docsrs , doc (cfg (feature = "time")))] $ item) * } }
};
}
