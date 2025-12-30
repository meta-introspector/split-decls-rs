// Generated macro for remove_capability_from_bounding_set (function)
macro_rules! Depcrate_thread_prctlremove_capability_from_bounding_set {
() => {
// Module: crate::thread::prctl
// Provides: {"remove_capability_from_bounding_set"}
// Dependencies: {}
# [doc = " If the calling thread has the [`Capability::SetPermittedCapabilities`]"] # [doc = " capability within its user namespace, then drop the specified capability"] # [doc = " from the thread's capability bounding set."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_CAPBSET_DROP,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_CAPBSET_DROP,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn remove_capability_from_bounding_set (capability : impl CompatCapability) -> io :: Result < () > { let capset = capability . as_capability_set (private :: Token) . bits () ; if capset . count_ones () != 1 { return Err (Errno :: INVAL) ; } let cap = capset . trailing_zeros () ; unsafe { prctl_2args (PR_CAPBSET_DROP , cap as usize as * mut _) } . map (| _r | ()) }
};
}
