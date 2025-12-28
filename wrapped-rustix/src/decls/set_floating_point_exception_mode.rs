macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_floating_point_exception_mode {
    () => {
        deps!();
        # [doc = " Set floating point exception mode."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_FPEXC,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_FPEXC,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_FPEXEC")] pub fn set_floating_point_exception_mode (config : Option < FloatingPointExceptionMode > ,) -> io :: Result < () > { let config = config . as_ref () . map_or (0 , FloatingPointExceptionMode :: bits) ; unsafe { prctl_2args (PR_SET_FPEXC , config as usize as * mut _) } . map (| _r | ()) }
    };
}

set_floating_point_exception_mode!();