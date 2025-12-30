// Generated macro for parent_process_death_signal (function)
macro_rules! Depcrate_process_procctlparent_process_death_signal {
() => {
// Module: crate::process::procctl
// Provides: {"parent_process_death_signal"}
// Dependencies: {}
# [doc = " Get the current value of the parent process death signal."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux: `prctl(PR_GET_PDEATHSIG,…)`]"] # [doc = "  - [FreeBSD: `procctl(PROC_PDEATHSIG_STATUS,…)`]"] # [doc = ""] # [doc = " [Linux: `prctl(PR_GET_PDEATHSIG,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [doc = " [FreeBSD: `procctl(PROC_PDEATHSIG_STATUS,…)`]: https://man.freebsd.org/cgi/man.cgi?query=procctl&sektion=2"] # [inline] pub fn parent_process_death_signal () -> io :: Result < Option < Signal > > { let raw = unsafe { procctl_get_optional :: < c_int > (PROC_PDEATHSIG_STATUS , None) } ? ; if let Some (non_zero) = NonZeroI32 :: new (raw) { Ok (Some (unsafe { Signal :: from_raw_nonzero_unchecked (non_zero) })) } else { Ok (None) } }
};
}
