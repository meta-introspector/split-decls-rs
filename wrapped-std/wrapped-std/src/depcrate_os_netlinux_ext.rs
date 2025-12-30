// Generated macro for linux_ext (module)
macro_rules! Depcrate_os_netlinux_ext {
() => {
// Module: crate::os::net
// Provides: {"linux_ext"}
// Dependencies: {}
# [cfg (not (all (doc , any (all (target_arch = "wasm32" , not (target_os = "wasi")) , all (target_vendor = "fortanix" , target_env = "sgx")))))] # [cfg (any (target_os = "linux" , target_os = "android" , target_os = "cygwin" , doc))] pub (super) mod linux_ext ;
};
}
