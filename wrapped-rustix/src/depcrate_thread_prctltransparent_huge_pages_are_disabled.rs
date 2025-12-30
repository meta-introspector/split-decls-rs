// Generated macro for transparent_huge_pages_are_disabled (function)
macro_rules! Depcrate_thread_prctltransparent_huge_pages_are_disabled {
() => {
// Module: crate::thread::prctl
// Provides: {"transparent_huge_pages_are_disabled"}
// Dependencies: {}
# [doc = " Get the current setting of the `THP disable` flag for the calling thread."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_THP_DISABLE,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_THP_DISABLE,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn transparent_huge_pages_are_disabled () -> io :: Result < bool > { unsafe { prctl_1arg (PR_GET_THP_DISABLE) } . map (| r | r != 0) }
};
}
