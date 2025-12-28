macro_rules! deps {
    () => {
        TimingMethod!();
        Result!();
    };
}

macro_rules! timing_method {
    () => {
        deps!();
        # [doc = " Get which process timing method is currently in use."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_TIMING,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_TIMING,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_GET_TIMING")] pub fn timing_method () -> io :: Result < TimingMethod > { unsafe { prctl_1arg (PR_GET_TIMING) } . and_then (TryInto :: try_into) }
    };
}

timing_method!();