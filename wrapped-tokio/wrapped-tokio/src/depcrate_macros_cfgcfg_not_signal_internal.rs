// Generated macro for cfg_not_signal_internal (macro)
macro_rules! Depcrate_macros_cfgcfg_not_signal_internal {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_signal_internal"}
// Dependencies: {}
macro_rules ! cfg_not_signal_internal { ($ ($ item : item) *) => { $ (# [cfg (any (loom , not (unix) , not (any (feature = "signal" , all (unix , feature = "process")))))] $ item) * } }
};
}
