// Generated macro for set_unaligned_access_control (function)
macro_rules! Depcrate_process_prctlset_unaligned_access_control {
() => {
// Module: crate::process::prctl
// Provides: {"set_unaligned_access_control"}
// Dependencies: {}
# [doc = " Set unaligned access control bits."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_UNALIGN,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_UNALIGN,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_UNALIGN")] pub fn set_unaligned_access_control (config : UnalignedAccessControl) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_UNALIGN , config . bits () as usize as * mut _) } . map (| _r | ()) }
};
}
