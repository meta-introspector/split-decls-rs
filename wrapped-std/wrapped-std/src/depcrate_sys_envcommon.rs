// Generated macro for common (module)
macro_rules! Depcrate_sys_envcommon {
() => {
// Module: crate::sys::env
// Provides: {"common"}
// Dependencies: {}
# [cfg (any (target_family = "unix" , target_os = "hermit" , all (target_vendor = "fortanix" , target_env = "sgx") , target_os = "solid_asp3" , target_os = "uefi" , target_os = "wasi" , target_os = "xous" ,))] mod common ;
};
}
