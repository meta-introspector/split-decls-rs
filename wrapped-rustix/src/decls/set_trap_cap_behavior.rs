macro_rules! deps {
    () => {
        ProcSelector!();
        Result!();
        TrapCapBehavior!();
    };
}

macro_rules! set_trap_cap_behavior {
    () => {
        deps!();
        # [doc = " Set the current value of the capability mode violation trapping behavior."] # [doc = ""] # [doc = " If this behavior is enabled, the kernel would deliver a [`Signal::Trap`]"] # [doc = " signal on any return from a system call that would result in a"] # [doc = " [`io::Errno::NOTCAPABLE`] or [`io::Errno::CAPMODE`] error."] # [doc = ""] # [doc = " This behavior is inherited by the children of the process and is kept"] # [doc = " across `execve` calls."] # [doc = ""] # [doc = " # References"] # [doc = "  - [FreeBSD: `procctl(PROC_TRAPCAP_CTL,…)`]"] # [doc = ""] # [doc = " [FreeBSD: `procctl(PROC_TRAPCAP_CTL,…)`]: https://man.freebsd.org/cgi/man.cgi?query=procctl&sektion=2"] # [inline] pub fn set_trap_cap_behavior (process : ProcSelector , config : TrapCapBehavior) -> io :: Result < () > { let config = config as c_int ; unsafe { procctl_set :: < c_int > (PROC_TRAPCAP_CTL , process , & config) } }
    };
}

set_trap_cap_behavior!();