// Generated macro for cfg_not_taskdump (macro)
macro_rules! Depcrate_macros_cfgcfg_not_taskdump {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_not_taskdump"}
// Dependencies: {}
macro_rules ! cfg_not_taskdump { ($ ($ item : item) *) => { $ (# [cfg (not (all (tokio_unstable , feature = "taskdump" , feature = "rt" , target_os = "linux" , any (target_arch = "aarch64" , target_arch = "x86" , target_arch = "x86_64"))))] $ item) * } ; }
};
}
