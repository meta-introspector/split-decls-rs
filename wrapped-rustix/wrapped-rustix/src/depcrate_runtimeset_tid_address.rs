// Generated macro for set_tid_address (function)
macro_rules! Depcrate_runtimeset_tid_address {
() => {
// Module: crate::runtime
// Provides: {"set_tid_address"}
// Dependencies: {}
# [doc = " Set the x86-64 thread ID address."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This is a very low-level feature for implementing threading libraries."] # [doc = " See the references links above."] # [inline] pub unsafe fn set_tid_address (data : * mut c_void) -> Pid { backend :: runtime :: syscalls :: tls :: set_tid_address (data) }
};
}
