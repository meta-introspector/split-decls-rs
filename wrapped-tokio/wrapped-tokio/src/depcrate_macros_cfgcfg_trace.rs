// Generated macro for cfg_trace (macro)
macro_rules! Depcrate_macros_cfgcfg_trace {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_trace"}
// Dependencies: {}
macro_rules ! cfg_trace { ($ ($ item : item) *) => { $ (# [cfg (all (tokio_unstable , feature = "tracing"))] # [cfg_attr (docsrs , doc (cfg (all (tokio_unstable , feature = "tracing"))))] $ item) * } ; }
};
}
