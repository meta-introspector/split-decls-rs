macro_rules! deps {
    () => {
        ProcSelector!();
        Result!();
        Pid!();
        KillResult!();
        Signal!();
    };
}

macro_rules! reaper_kill {
    () => {
        deps!();
        # [doc = " Deliver a signal to some subset of the descendants of the reaper."] # [doc = ""] # [doc = " # References"] # [doc = "  - [FreeBSD: `procctl(PROC_REAP_KILL,…)`]"] # [doc = ""] # [doc = " [FreeBSD: `procctl(PROC_REAP_KILL,…)`]: https://man.freebsd.org/cgi/man.cgi?query=procctl&sektion=2"] pub fn reaper_kill (process : ProcSelector , signal : Signal , direct_children : bool , subtree : Option < Pid > ,) -> io :: Result < KillResult > { let mut flags = KillFlags :: empty () ; flags . set (KillFlags :: CHILDREN , direct_children) ; flags . set (KillFlags :: SUBTREE , subtree . is_some ()) ; let mut req = procctl_reaper_kill { rk_sig : signal . as_raw () , rk_flags : flags . bits () , rk_subtree : subtree . map (| p | p . as_raw_nonzero () . into ()) . unwrap_or (0) , rk_killed : 0 , rk_fpid : 0 , rk_pad0 : [0 ; 15] , } ; unsafe { procctl (PROC_REAP_KILL , process , as_mut_ptr (& mut req) . cast ()) ? } ; Ok (KillResult { killed : req . rk_killed as _ , first_failed : Pid :: from_raw (req . rk_fpid) , }) }
    };
}

reaper_kill!()