// Generated macro for configure_capability_in_ambient_set (function)
macro_rules! Depcrate_thread_prctlconfigure_capability_in_ambient_set {
() => {
// Module: crate::thread::prctl
// Provides: {"configure_capability_in_ambient_set"}
// Dependencies: {}
# [doc = " Add or remove the specified capability to the ambient set."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_CAP_AMBIENT,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_CAP_AMBIENT,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn configure_capability_in_ambient_set (capability : impl CompatCapability , enable : bool ,) -> io :: Result < () > { let sub_operation = if enable { PR_CAP_AMBIENT_RAISE } else { PR_CAP_AMBIENT_LOWER } ; let capset = capability . as_capability_set (private :: Token) . bits () ; if capset . count_ones () != 1 { return Err (Errno :: INVAL) ; } let cap = capset . trailing_zeros () ; unsafe { prctl_3args (PR_CAP_AMBIENT , sub_operation as * mut _ , cap as usize as * mut _ ,) } . map (| _r | ()) }
};
}
