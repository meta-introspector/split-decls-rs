// Generated macro for get_clear_child_tid_address (function)
macro_rules! Depcrate_thread_prctlget_clear_child_tid_address {
() => {
// Module: crate::thread::prctl
// Provides: {"get_clear_child_tid_address"}
// Dependencies: {}
# [doc = " Get the `clear_child_tid` address set by `set_tid_address`"] # [doc = " and `clone`'s `CLONE_CHILD_CLEARTID` flag."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_TID_ADDRESS,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_TID_ADDRESS,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn get_clear_child_tid_address () -> io :: Result < Option < NonNull < c_void > > > { unsafe { prctl_get_at_arg2_optional :: < * mut c_void > (PR_GET_TID_ADDRESS) } . map (NonNull :: new) }
};
}
