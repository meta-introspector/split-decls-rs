// Generated macro for configure_virtual_memory_map (function)
macro_rules! Depcrate_process_prctlconfigure_virtual_memory_map {
() => {
// Module: crate::process::prctl
// Provides: {"configure_virtual_memory_map"}
// Dependencies: {}
# [doc = " Provides one-shot access to all the addresses by passing in a"] # [doc = " [`PrctlMmMap`]."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_MM,PR_SET_MM_MAP,…)`]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Please ensure the conditions necessary to safely call this function, as"] # [doc = " detailed in the references above."] # [doc = ""] # [doc = " [`prctl(PR_SET_MM,PR_SET_MM_MAP,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_MM")] # [doc (alias = "PR_SET_MM_MAP")] pub unsafe fn configure_virtual_memory_map (config : & PrctlMmMap) -> io :: Result < () > { syscalls :: prctl (PR_SET_MM , PR_SET_MM_MAP as * mut _ , as_ptr (config) as * mut _ , size_of :: < PrctlMmMap > () as * mut _ , null_mut () ,) . map (| _r | ()) }
};
}
