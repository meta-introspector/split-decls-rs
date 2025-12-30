// Generated macro for no_new_privs (function)
macro_rules! Depcrate_process_procctlno_new_privs {
() => {
// Module: crate::process::procctl
// Provides: {"no_new_privs"}
// Dependencies: {}
# [doc = " Check the `no_new_privs` mode of the specified process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux: `prctl(PR_GET_NO_NEW_PRIVS,…)`]"] # [doc = "  - [FreeBSD: `procctl(PROC_NO_NEW_PRIVS_STATUS,…)`]"] # [doc = ""] # [doc = " [Linux: `prctl(PR_GET_NO_NEW_PRIVS,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [doc = " [FreeBSD: `procctl(PROC_NO_NEW_PRIVS_STATUS,…)`]: https://man.freebsd.org/cgi/man.cgi?query=procctl&sektion=2"] # [inline] pub fn no_new_privs (process : ProcSelector) -> io :: Result < bool > { unsafe { procctl_get_optional :: < c_int > (PROC_NO_NEW_PRIVS_STATUS , process) } . map (| x | x == PROC_NO_NEW_PRIVS_ENABLE) }
};
}
