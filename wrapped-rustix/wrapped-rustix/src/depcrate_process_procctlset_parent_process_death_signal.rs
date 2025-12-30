// Generated macro for set_parent_process_death_signal (function)
macro_rules! Depcrate_process_procctlset_parent_process_death_signal {
() => {
// Module: crate::process::procctl
// Provides: {"set_parent_process_death_signal"}
// Dependencies: {}
# [doc = " Set the parent-death signal of the calling process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux: `prctl(PR_SET_PDEATHSIG,…)`]"] # [doc = "  - [FreeBSD: `procctl(PROC_PDEATHSIG_CTL,…)`]"] # [doc = ""] # [doc = " [Linux: `prctl(PR_SET_PDEATHSIG,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [doc = " [FreeBSD: `procctl(PROC_PDEATHSIG_CTL,…)`]: https://man.freebsd.org/cgi/man.cgi?query=procctl&sektion=2"] # [inline] pub fn set_parent_process_death_signal (signal : Option < Signal >) -> io :: Result < () > { let signal = signal . map_or (0 , | signal | signal . as_raw ()) ; unsafe { procctl_set :: < c_int > (PROC_PDEATHSIG_CTL , None , & signal) } }
};
}
