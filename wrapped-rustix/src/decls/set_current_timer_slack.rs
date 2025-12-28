macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_current_timer_slack {
    () => {
        deps!();
        # [doc = " Sets the `current` timer slack value for the calling thread."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_TIMERSLACK,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_TIMERSLACK,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn set_current_timer_slack (value : Option < NonZeroU64 >) -> io :: Result < () > { let value = usize :: try_from (value . map_or (0 , NonZeroU64 :: get)) . map_err (| _r | io :: Errno :: RANGE) ? ; unsafe { prctl_2args (PR_SET_TIMERSLACK , value as * mut _) } . map (| _r | ()) }
    };
}

set_current_timer_slack!()