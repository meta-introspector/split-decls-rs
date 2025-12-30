// Generated macro for current_timer_slack (function)
macro_rules! Depcrate_thread_prctlcurrent_timer_slack {
() => {
// Module: crate::thread::prctl
// Provides: {"current_timer_slack"}
// Dependencies: {}
# [doc = " Get the `current` timer slack value of the calling thread."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_TIMERSLACK,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_TIMERSLACK,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn current_timer_slack () -> io :: Result < u64 > { unsafe { prctl_1arg (PR_GET_TIMERSLACK) } . map (| r | r as u64) }
};
}
