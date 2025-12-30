// Generated macro for set_capabilities_secure_bits (function)
macro_rules! Depcrate_thread_prctlset_capabilities_secure_bits {
() => {
// Module: crate::thread::prctl
// Provides: {"set_capabilities_secure_bits"}
// Dependencies: {}
# [doc = " Set the `securebits` flags of the calling thread."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_SECUREBITS,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_SECUREBITS,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn set_capabilities_secure_bits (bits : CapabilitiesSecureBits) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_SECUREBITS , bits . bits () as usize as * mut _) } . map (| _r | ()) }
};
}
