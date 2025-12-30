// Generated macro for cfg_io_uring (macro)
macro_rules! Depcrate_macros_cfgcfg_io_uring {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_io_uring"}
// Dependencies: {}
macro_rules ! cfg_io_uring { ($ ($ item : item) *) => { $ (# [cfg (all (tokio_unstable , feature = "io-uring" , feature = "rt" , feature = "fs" , target_os = "linux" ,))] $ item) * } ; }
};
}
