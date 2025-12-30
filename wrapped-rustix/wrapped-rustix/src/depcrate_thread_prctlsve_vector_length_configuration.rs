// Generated macro for sve_vector_length_configuration (function)
macro_rules! Depcrate_thread_prctlsve_vector_length_configuration {
() => {
// Module: crate::thread::prctl
// Provides: {"sve_vector_length_configuration"}
// Dependencies: {}
# [doc = " Get the thread's current SVE vector length configuration."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SVE_GET_VL,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SVE_GET_VL,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn sve_vector_length_configuration () -> io :: Result < SVEVectorLengthConfig > { let bits = unsafe { prctl_1arg (PR_SVE_GET_VL) ? } as c_uint ; Ok (SVEVectorLengthConfig { vector_length_in_bytes : bits & PR_SVE_VL_LEN_MASK , vector_length_inherited_across_execve : (bits & PR_SVE_VL_INHERIT) != 0 , }) }
};
}
