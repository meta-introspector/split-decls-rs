macro_rules! deps {
    () => {
        Result!();
        FloatingPointMode!();
    };
}

macro_rules! set_floating_point_mode {
    () => {
        deps!();
        # [doc = " Allow control of the floating point mode from user space."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_FP_MODE,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_FP_MODE,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_FP_MODE")] pub fn set_floating_point_mode (mode : FloatingPointMode) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_FP_MODE , mode as usize as * mut _) } . map (| _r | ()) }
    };
}

set_floating_point_mode!();