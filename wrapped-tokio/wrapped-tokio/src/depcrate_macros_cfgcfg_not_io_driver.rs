// Generated macro for cfg_not_io_driver (macro)
macro_rules! Depcrate_macros_cfgcfg_not_io_driver {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_io_driver"}
// Dependencies: {}
macro_rules ! cfg_not_io_driver { ($ ($ item : item) *) => { $ (# [cfg (not (any (feature = "net" , all (unix , feature = "process") , all (unix , feature = "signal") , all (tokio_unstable , feature = "io-uring" , feature = "rt" , feature = "fs" , target_os = "linux"))))] $ item) * } }
};
}
