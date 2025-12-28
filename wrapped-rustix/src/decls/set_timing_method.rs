macro_rules! deps {
    () => {
        Result!();
        TimingMethod!();
    };
}

macro_rules! set_timing_method {
    () => {
        deps!();
        # [doc = " Set whether to use (normal, traditional) statistical process timing or"] # [doc = " accurate timestamp-based process timing."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_TIMING,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_TIMING,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_TIMING")] pub fn set_timing_method (method : TimingMethod) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_TIMING , method as usize as * mut _) } . map (| _r | ()) }
    };
}

set_timing_method!()