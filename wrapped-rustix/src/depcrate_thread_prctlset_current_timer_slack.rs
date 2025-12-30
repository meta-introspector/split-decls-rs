// Generated macro for set_current_timer_slack (function)
macro_rules! Depcrate_thread_prctlset_current_timer_slack {
() => {
// Module: crate::thread::prctl
// Provides: {"set_current_timer_slack"}
// Dependencies: {}
# [doc = " Sets the `current` timer slack value for the calling thread."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_TIMERSLACK,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_TIMERSLACK,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn set_current_timer_slack (value : Option < NonZeroU64 >) -> io :: Result < () > { let value = usize :: try_from (value . map_or (0 , NonZeroU64 :: get)) . map_err (| _r | io :: Errno :: RANGE) ? ; unsafe { prctl_2args (PR_SET_TIMERSLACK , value as * mut _) } . map (| _r | ()) }
};
}
