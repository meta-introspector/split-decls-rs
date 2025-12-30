// Generated macro for cfg_not_trace (macro)
macro_rules! Depcrate_macros_cfgcfg_not_trace {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_trace"}
// Dependencies: {}
macro_rules ! cfg_not_trace { ($ ($ item : item) *) => { $ (# [cfg (any (not (tokio_unstable) , not (feature = "tracing")))] $ item) * } }
};
}
