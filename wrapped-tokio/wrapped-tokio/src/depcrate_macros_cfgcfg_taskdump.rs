// Generated macro for cfg_taskdump (macro)
macro_rules! Depcrate_macros_cfgcfg_taskdump {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_taskdump"}
// Dependencies: {}
macro_rules ! cfg_taskdump { ($ ($ item : item) *) => { $ (# [cfg (all (tokio_unstable , feature = "taskdump" , feature = "rt" , target_os = "linux" , any (target_arch = "aarch64" , target_arch = "x86" , target_arch = "x86_64")))] $ item) * } ; }
};
}
