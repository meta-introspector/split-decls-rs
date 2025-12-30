// Generated macro for cfg_signal_internal (macro)
macro_rules! Depcrate_macros_cfgcfg_signal_internal {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_signal_internal"}
// Dependencies: {}
macro_rules ! cfg_signal_internal { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "signal" , all (unix , feature = "process")))] # [cfg (not (loom))] $ item) * } }
};
}
