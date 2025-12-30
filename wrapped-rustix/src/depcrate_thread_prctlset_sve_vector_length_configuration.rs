// Generated macro for set_sve_vector_length_configuration (function)
macro_rules! Depcrate_thread_prctlset_sve_vector_length_configuration {
() => {
// Module: crate::thread::prctl
// Provides: {"set_sve_vector_length_configuration"}
// Dependencies: {}
# [doc = " Configure the thread's vector length of Scalable Vector Extension."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SVE_SET_VL,…)`]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Please ensure the conditions necessary to safely call this function,"] # [doc = " as detailed in the references above."] # [doc = ""] # [doc = " [`prctl(PR_SVE_SET_VL,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub unsafe fn set_sve_vector_length_configuration (vector_length_in_bytes : usize , vector_length_inherited_across_execve : bool , defer_change_to_next_execve : bool ,) -> io :: Result < () > { let vector_length_in_bytes = u32 :: try_from (vector_length_in_bytes) . map_err (| _r | io :: Errno :: RANGE) ? ; let mut bits = vector_length_in_bytes & PR_SVE_VL_LEN_MASK ; if vector_length_inherited_across_execve { bits |= PR_SVE_VL_INHERIT ; } if defer_change_to_next_execve { bits |= PR_SVE_SET_VL_ONEXEC ; } prctl_2args (PR_SVE_SET_VL , bits as usize as * mut _) . map (| _r | ()) }
};
}
