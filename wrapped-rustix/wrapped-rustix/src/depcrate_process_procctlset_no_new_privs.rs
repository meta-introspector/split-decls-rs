// Generated macro for set_no_new_privs (function)
macro_rules! Depcrate_process_procctlset_no_new_privs {
() => {
// Module: crate::process::procctl
// Provides: {"set_no_new_privs"}
// Dependencies: {}
# [doc = " Enable the `no_new_privs` mode that ignores SUID and SGID bits on `execve`"] # [doc = " in the specified process and its future descendants."] # [doc = ""] # [doc = " This is similar to `set_no_new_privs` on Linux, with the exception that on"] # [doc = " FreeBSD there is no argument `no_new_privs` argument as it's only possible"] # [doc = " to enable this mode and there's no going back."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux: `prctl(PR_SET_NO_NEW_PRIVS,…)`]"] # [doc = "  - [FreeBSD: `procctl(PROC_NO_NEW_PRIVS_CTL,…)`]"] # [doc = ""] # [doc = " [Linux: `prctl(PR_SET_NO_NEW_PRIVS,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [doc = " [FreeBSD: `procctl(PROC_NO_NEW_PRIVS_CTL,…)`]: https://man.freebsd.org/cgi/man.cgi?query=procctl&sektion=2"] # [inline] pub fn set_no_new_privs (process : ProcSelector) -> io :: Result < () > { unsafe { procctl_set :: < c_int > (PROC_NO_NEW_PRIVS_CTL , process , & PROC_NO_NEW_PRIVS_ENABLE) } }
};
}
