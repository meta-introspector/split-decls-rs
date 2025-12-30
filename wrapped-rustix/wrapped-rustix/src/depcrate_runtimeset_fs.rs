// Generated macro for set_fs (function)
macro_rules! Depcrate_runtimeset_fs {
() => {
// Module: crate::runtime
// Provides: {"set_fs"}
// Dependencies: {}
# [doc = " `prctl(PR_SET_FS, data)`—Set the x86-64 `fs` register."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This is a very low-level feature for implementing threading libraries."] # [doc = " See the references links above."] # [cfg (target_arch = "x86_64")] # [inline] pub unsafe fn set_fs (data : * mut c_void) { backend :: runtime :: syscalls :: tls :: set_fs (data) }
};
}
