// Generated macro for cfg_io_blocking (macro)
macro_rules! Depcrate_macros_cfgcfg_io_blocking {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_io_blocking"}
// Dependencies: {}
macro_rules ! cfg_io_blocking { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "io-std" , feature = "fs" , all (windows , feature = "process") ,))] $ item) * } }
};
}
