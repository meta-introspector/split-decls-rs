// Generated macro for set_thread_area (function)
macro_rules! Depcrate_runtimeset_thread_area {
() => {
// Module: crate::runtime
// Provides: {"set_thread_area"}
// Dependencies: {}
# [cfg (target_arch = "x86")] # [inline] pub unsafe fn set_thread_area (u_info : & mut UserDesc) -> io :: Result < () > { backend :: runtime :: syscalls :: tls :: set_thread_area (u_info) }
};
}
