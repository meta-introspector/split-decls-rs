macro_rules! deps {
    () => {
        TrapCapBehavior!();
        Result!();
        ProcSelector!();
    };
}

macro_rules! trap_cap_behavior {
    () => {
        deps!();
        # [doc = " Get the current value of the capability mode violation trapping behavior."] # [doc = ""] # [doc = " # References"] # [doc = "  - [FreeBSD: `procctl(PROC_TRAPCAP_STATUS,…)`]"] # [doc = ""] # [doc = " [FreeBSD: `procctl(PROC_TRAPCAP_STATUS,…)`]: https://man.freebsd.org/cgi/man.cgi?query=procctl&sektion=2"] # [inline] pub fn trap_cap_behavior (process : ProcSelector) -> io :: Result < TrapCapBehavior > { let val = unsafe { procctl_get_optional :: < c_int > (PROC_TRAPCAP_STATUS , process) } ? ; match val { PROC_TRAPCAP_CTL_DISABLE => Ok (TrapCapBehavior :: Disable) , PROC_TRAPCAP_CTL_ENABLE => Ok (TrapCapBehavior :: Enable) , _ => Err (io :: Errno :: RANGE) , } }
    };
}

trap_cap_behavior!()