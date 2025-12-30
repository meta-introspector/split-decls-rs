// Generated macro for disable_syscall_user_dispatch (function)
macro_rules! Depcrate_thread_prctldisable_syscall_user_dispatch {
() => {
// Module: crate::thread::prctl
// Provides: {"disable_syscall_user_dispatch"}
// Dependencies: {}
# [doc = " Disable Syscall User Dispatch mechanism."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_SYSCALL_USER_DISPATCH,PR_SYS_DISPATCH_OFF,…)`]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Please ensure the conditions necessary to safely call this function, as"] # [doc = " detailed in the references above."] # [doc = ""] # [doc = " [`prctl(PR_SET_SYSCALL_USER_DISPATCH,PR_SYS_DISPATCH_OFF,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub unsafe fn disable_syscall_user_dispatch () -> io :: Result < () > { prctl_2args (PR_SET_SYSCALL_USER_DISPATCH , PR_SYS_DISPATCH_OFF as * mut _) . map (| _r | ()) }
};
}
