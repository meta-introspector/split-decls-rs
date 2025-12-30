// Generated macro for disable_transparent_huge_pages (function)
macro_rules! Depcrate_thread_prctldisable_transparent_huge_pages {
() => {
// Module: crate::thread::prctl
// Provides: {"disable_transparent_huge_pages"}
// Dependencies: {}
# [doc = " Set the state of the `THP disable` flag for the calling thread."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_THP_DISABLE,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_THP_DISABLE,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn disable_transparent_huge_pages (thp_disable : bool) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_THP_DISABLE , usize :: from (thp_disable) as * mut _) } . map (| _r | ()) }
};
}
