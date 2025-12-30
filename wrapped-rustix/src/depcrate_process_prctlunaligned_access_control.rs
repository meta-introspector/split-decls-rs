// Generated macro for unaligned_access_control (function)
macro_rules! Depcrate_process_prctlunaligned_access_control {
() => {
// Module: crate::process::prctl
// Provides: {"unaligned_access_control"}
// Dependencies: {}
# [doc = " Get unaligned access control bits."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_UNALIGN,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_UNALIGN,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_GET_UNALIGN")] pub fn unaligned_access_control () -> io :: Result < UnalignedAccessControl > { let r = unsafe { prctl_get_at_arg2_optional :: < c_uint > (PR_GET_UNALIGN) ? } ; UnalignedAccessControl :: from_bits (r) . ok_or (io :: Errno :: RANGE) }
};
}
