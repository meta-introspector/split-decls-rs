// Generated macro for configure_io_flusher_behavior (function)
macro_rules! Depcrate_process_prctlconfigure_io_flusher_behavior {
() => {
// Module: crate::process::prctl
// Provides: {"configure_io_flusher_behavior"}
// Dependencies: {}
# [doc = " Put the process in the `IO_FLUSHER` state, allowing it to make progress"] # [doc = " when allocating memory."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_IO_FLUSHER,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_IO_FLUSHER,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_IO_FLUSHER")] pub fn configure_io_flusher_behavior (enable : bool) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_IO_FLUSHER , usize :: from (enable) as * mut _) } . map (| _r | ()) }
};
}
