// Generated macro for arm_set_tls (function)
macro_rules! Depcrate_runtimearm_set_tls {
() => {
// Module: crate::runtime
// Provides: {"arm_set_tls"}
// Dependencies: {}
# [cfg (target_arch = "arm")] # [inline] pub unsafe fn arm_set_tls (data : * mut c_void) -> io :: Result < () > { backend :: runtime :: syscalls :: tls :: arm_set_tls (data) }
};
}
