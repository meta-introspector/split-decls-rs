// Generated macro for cfg_io_driver_impl (macro)
macro_rules! Depcrate_macros_cfgcfg_io_driver_impl {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_io_driver_impl"}
// Dependencies: {}
macro_rules ! cfg_io_driver_impl { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "net" , all (unix , feature = "process") , all (unix , feature = "signal") , all (tokio_unstable , feature = "io-uring" , feature = "rt" , feature = "fs" , target_os = "linux")))] $ item) * } }
};
}
