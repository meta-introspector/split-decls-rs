// Generated macro for unshare (function)
macro_rules! Depcrate_thread_setnsunshare {
() => {
// Module: crate::thread::setns
// Provides: {"unshare"}
// Dependencies: {}
# [doc = " `unshare(flags)`—Deprecated in favor of [`unshare_unsafe`]."] # [doc = ""] # [doc = " This function should be unsafe; see the safety comment on `unshare_unsafe`."] # [deprecated (since = "1.1.0" , note = "Use `unshare_unsafe`")] pub fn unshare (flags : UnshareFlags) -> io :: Result < () > { unsafe { syscalls :: unshare (flags) } }
};
}
