macro_rules! deps {
    () => {
        Result!();
        TimeStampCounterReadability!();
    };
}

macro_rules! time_stamp_counter_readability {
    () => {
        deps!();
        # [doc = " Get the state of the flag determining if the timestamp counter can be read."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_TSC,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_TSC,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_GET_TSC")] pub fn time_stamp_counter_readability () -> io :: Result < TimeStampCounterReadability > { unsafe { prctl_get_at_arg2 :: < c_uint , _ > (PR_GET_TSC) } }
    };
}

time_stamp_counter_readability!()