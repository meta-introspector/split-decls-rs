// Generated macro for cfg_signal (macro)
macro_rules! Depcrate_macroscfg_signal {
() => {
// Module: crate::macros
// Provides: {"cfg_signal"}
// Dependencies: {}
macro_rules ! cfg_signal { ($ ($ item : item) *) => { $ (# [cfg (feature = "signal")] # [cfg_attr (docsrs , doc (cfg (feature = "signal")))] $ item) * } }
};
}
