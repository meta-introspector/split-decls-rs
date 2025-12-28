macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_reaper_status {
    () => {
        deps!();
        # [doc = " Acquire or release the reaper status of the calling process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [FreeBSD: `procctl(PROC_REAP_ACQUIRE/RELEASE,…)`]"] # [doc = ""] # [doc = " [FreeBSD: `procctl(PROC_REAP_ACQUIRE/RELEASE,…)`]: https://man.freebsd.org/cgi/man.cgi?query=procctl&sektion=2"] # [inline] pub fn set_reaper_status (reaper : bool) -> io :: Result < () > { unsafe { procctl (if reaper { PROC_REAP_ACQUIRE } else { PROC_REAP_RELEASE } , None , ptr :: null_mut () ,) } }
    };
}

set_reaper_status!()