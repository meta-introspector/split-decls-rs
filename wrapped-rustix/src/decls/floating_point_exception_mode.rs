macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! floating_point_exception_mode {
    () => {
        deps!();
        # [doc = " Get floating point exception mode."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_FPEXC,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_FPEXC,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_GET_FPEXEC")] pub fn floating_point_exception_mode () -> io :: Result < Option < FloatingPointExceptionMode > > { unsafe { prctl_get_at_arg2_optional :: < c_uint > (PR_GET_FPEXC) } . map (FloatingPointExceptionMode :: from_bits) }
    };
}

floating_point_exception_mode!();