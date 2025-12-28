macro_rules! deps {
    () => {
        Result!();
        FloatingPointMode!();
    };
}

macro_rules! floating_point_mode {
    () => {
        deps!();
        # [doc = " Get the current floating point mode."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_FP_MODE,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_FP_MODE,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_GET_FP_MODE")] pub fn floating_point_mode () -> io :: Result < FloatingPointMode > { let r = unsafe { prctl_1arg (PR_GET_FP_MODE) ? } as c_uint ; FloatingPointMode :: try_from (r) }
    };
}

floating_point_mode!()