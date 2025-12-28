macro_rules! deps {
    () => {
        Pid!();
        ProcSelector!();
        Result!();
        ReaperStatus!();
    };
}

macro_rules! get_reaper_status {
    () => {
        deps!();
        # [doc = " Get information about the reaper of the specified process (or the process"] # [doc = " itself if it is a reaper)."] # [doc = ""] # [doc = " # References"] # [doc = "  - [FreeBSD: `procctl(PROC_REAP_STATUS,…)`]"] # [doc = ""] # [doc = " [FreeBSD: `procctl(PROC_REAP_STATUS,…)`]: https://man.freebsd.org/cgi/man.cgi?query=procctl&sektion=2"] # [inline] pub fn get_reaper_status (process : ProcSelector) -> io :: Result < ReaperStatus > { let raw = unsafe { procctl_get_optional :: < procctl_reaper_status > (PROC_REAP_STATUS , process) } ? ; Ok (ReaperStatus { flags : ReaperStatusFlags :: from_bits_retain (raw . rs_flags) , children : raw . rs_children as _ , descendants : raw . rs_descendants as _ , reaper : Pid :: from_raw (raw . rs_reaper) . ok_or (io :: Errno :: RANGE) ? , pid : if raw . rs_pid == - 1 { None } else { Some (Pid :: from_raw (raw . rs_pid) . ok_or (io :: Errno :: RANGE) ?) } , }) }
    };
}

get_reaper_status!();